use application::authorization::policy::{AuthorizationPolicy, AuthorizationPolicyId};
use application::authorization::repository_port::AuthorizationPolicyRepositoryPort;
use application::errors::ServiceError;
use async_trait::async_trait;
use casbin::{Adapter, Filter, Model, Result as CasbinResult};
use redis::AsyncCommands;
use sha2::{Digest, Sha256};

use crate::adapters::redis::client::RedisClient;

#[derive(Clone)]
pub struct RedisPolicyStoreAdapter {
    client: RedisClient,
    namespace: String,
}

impl RedisPolicyStoreAdapter {
    pub fn new(client: RedisClient) -> Self {
        Self::with_namespace(client, "authorization")
    }

    pub fn with_namespace(client: RedisClient, namespace: impl Into<String>) -> Self {
        Self {
            client,
            namespace: namespace.into(),
        }
    }

    fn namespace(&self) -> &str {
        &self.namespace
    }
}

#[async_trait]
impl AuthorizationPolicyRepositoryPort for RedisPolicyStoreAdapter {
    async fn upsert(&self, policy: AuthorizationPolicy) -> Result<(), ServiceError> {
        let key = policy_key(self.namespace(), &policy.id);
        let type_index_key = policy_index_key(self.namespace(), &policy.policy_type);
        let all_index_key = policy_all_index_key(self.namespace());
        let types_index_key = policy_types_index_key(self.namespace());
        let payload = serde_json::to_string(&policy).map_err(ServiceError::internal)?;

        let mut connection = self.client.connection().map_err(|error| {
            tracing::error!(
                error = ?error,
                redis_url = %self.client.url(),
                operation = "connect",
                "Failed to acquire Redis connection for policy write."
            );
            ServiceError::internal(error)
        })?;

        let _: () = connection.set(&key, payload).await.map_err(|error| {
            tracing::error!(
                error = ?error,
                redis_url = %self.client.url(),
                operation = "set",
                key = %key,
                "Failed to persist authorization policy into Redis."
            );
            ServiceError::internal(error)
        })?;

        let _: () = connection
            .sadd(&type_index_key, &key)
            .await
            .map_err(|error| {
                tracing::error!(
                    error = ?error,
                    redis_url = %self.client.url(),
                    operation = "sadd",
                    key = %type_index_key,
                    policy_key = %key,
                    "Failed to update Redis policy index."
                );
                ServiceError::internal(error)
            })?;

        let _: () = connection
            .sadd(&all_index_key, &key)
            .await
            .map_err(|error| {
                tracing::error!(
                    error = ?error,
                    redis_url = %self.client.url(),
                    operation = "sadd",
                    key = %all_index_key,
                    policy_key = %key,
                    "Failed to update Redis policy registry."
                );
                ServiceError::internal(error)
            })?;

        let _: () = connection
            .sadd(&types_index_key, &policy.policy_type)
            .await
            .map_err(|error| {
                tracing::error!(
                    error = ?error,
                    redis_url = %self.client.url(),
                    operation = "sadd",
                    key = %types_index_key,
                    policy_type = %policy.policy_type,
                    "Failed to update Redis policy type registry."
                );
                ServiceError::internal(error)
            })?;

        Ok(())
    }

    async fn delete(&self, policy_id: AuthorizationPolicyId) -> Result<bool, ServiceError> {
        let key = policy_key(self.namespace(), &policy_id);
        let mut connection = self.client.connection().map_err(|error| {
            tracing::error!(
                error = ?error,
                redis_url = %self.client.url(),
                operation = "connect",
                "Failed to acquire Redis connection for policy delete."
            );
            ServiceError::internal(error)
        })?;

        let Some(policy) = fetch_policy(self.namespace(), &mut connection, &key).await? else {
            return Ok(false);
        };

        let deleted: i64 = connection.del(&key).await.map_err(|error| {
            tracing::error!(
                error = ?error,
                redis_url = %self.client.url(),
                operation = "del",
                key = %key,
                "Failed to delete authorization policy from Redis."
            );
            ServiceError::internal(error)
        })?;

        if deleted > 0 {
            remove_from_indexes(self.namespace(), &mut connection, &key, &policy.policy_type)
                .await?;
        }

        Ok(deleted > 0)
    }

    async fn list(
        &self,
        policy_type: Option<&str>,
    ) -> Result<Vec<AuthorizationPolicy>, ServiceError> {
        let mut connection = self.client.connection().map_err(|error| {
            tracing::error!(
                error = ?error,
                redis_url = %self.client.url(),
                operation = "connect",
                "Failed to acquire Redis connection for policy listing."
            );
            ServiceError::internal(error)
        })?;

        let keys: Vec<String> = if let Some(policy_type) = policy_type {
            let index_key = policy_index_key(self.namespace(), policy_type);
            connection.smembers(index_key).await.map_err(|error| {
                tracing::error!(
                    error = ?error,
                    redis_url = %self.client.url(),
                    operation = "smembers",
                    "Failed to read Redis policy index."
                );
                ServiceError::internal(error)
            })?
        } else {
            connection
                .smembers(policy_all_index_key(self.namespace()))
                .await
                .map_err(|error| {
                    tracing::error!(
                        error = ?error,
                        redis_url = %self.client.url(),
                        operation = "smembers",
                        "Failed to read Redis policy registry."
                    );
                    ServiceError::internal(error)
                })?
        };

        let mut policies = Vec::with_capacity(keys.len());
        for key in keys {
            let Some(policy) = fetch_policy(self.namespace(), &mut connection, &key).await? else {
                continue;
            };
            policies.push(policy);
        }

        Ok(policies)
    }

    async fn clear(&self, policy_type: Option<&str>) -> Result<u64, ServiceError> {
        let mut connection = self.client.connection().map_err(|error| {
            tracing::error!(
                error = ?error,
                redis_url = %self.client.url(),
                operation = "connect",
                "Failed to acquire Redis connection for policy cleanup."
            );
            ServiceError::internal(error)
        })?;

        match policy_type {
            Some(policy_type) => {
                let index_key = policy_index_key(self.namespace(), policy_type);
                let keys: Vec<String> = connection.smembers(&index_key).await.map_err(|error| {
                    tracing::error!(
                        error = ?error,
                        redis_url = %self.client.url(),
                        operation = "smembers",
                        key = %index_key,
                        "Failed to read Redis policy index."
                    );
                    ServiceError::internal(error)
                })?;

                let mut deleted = 0_u64;
                for key in keys {
                    let removed: i64 = connection.del(&key).await.map_err(|error| {
                        tracing::error!(
                            error = ?error,
                            redis_url = %self.client.url(),
                            operation = "del",
                            key = %key,
                            "Failed to delete authorization policy from Redis."
                        );
                        ServiceError::internal(error)
                    })?;
                    deleted += removed as u64;
                    let _: () = connection
                        .srem(policy_all_index_key(self.namespace()), &key)
                        .await
                        .map_err(|error| {
                            tracing::error!(
                                error = ?error,
                                redis_url = %self.client.url(),
                                operation = "srem",
                                key = %key,
                                "Failed to update Redis policy registry."
                            );
                            ServiceError::internal(error)
                        })?;
                }

                let _: () = connection.del(&index_key).await.map_err(|error| {
                    tracing::error!(
                        error = ?error,
                        redis_url = %self.client.url(),
                        operation = "del",
                        key = %index_key,
                        "Failed to delete Redis policy type index."
                    );
                    ServiceError::internal(error)
                })?;

                Ok(deleted)
            }
            None => {
                let keys: Vec<String> = connection
                    .smembers(policy_all_index_key(self.namespace()))
                    .await
                    .map_err(|error| {
                        tracing::error!(
                            error = ?error,
                            redis_url = %self.client.url(),
                            operation = "smembers",
                            "Failed to read Redis policy registry."
                        );
                        ServiceError::internal(error)
                    })?;

                let types: Vec<String> = connection
                    .smembers(policy_types_index_key(self.namespace()))
                    .await
                    .map_err(|error| {
                        tracing::error!(
                            error = ?error,
                            redis_url = %self.client.url(),
                            operation = "smembers",
                            "Failed to read Redis policy type registry."
                        );
                        ServiceError::internal(error)
                    })?;

                let mut deleted = 0_u64;
                for key in keys {
                    let removed: i64 = connection.del(&key).await.map_err(|error| {
                        tracing::error!(
                            error = ?error,
                            redis_url = %self.client.url(),
                            operation = "del",
                            key = %key,
                            "Failed to delete authorization policy from Redis."
                        );
                        ServiceError::internal(error)
                    })?;
                    deleted += removed as u64;
                }

                let _: () = connection
                    .del(policy_all_index_key(self.namespace()))
                    .await
                    .map_err(|error| {
                        tracing::error!(
                            error = ?error,
                            redis_url = %self.client.url(),
                            operation = "del",
                            "Failed to delete Redis policy registry."
                        );
                        ServiceError::internal(error)
                    })?;

                let _: () = connection
                    .del(policy_types_index_key(self.namespace()))
                    .await
                    .map_err(|error| {
                        tracing::error!(
                            error = ?error,
                            redis_url = %self.client.url(),
                            operation = "del",
                            "Failed to delete Redis policy type registry."
                        );
                        ServiceError::internal(error)
                    })?;

                for policy_type in types {
                    let _: () = connection
                        .del(policy_index_key(self.namespace(), &policy_type))
                        .await
                        .map_err(|error| {
                            tracing::error!(
                                error = ?error,
                                redis_url = %self.client.url(),
                                operation = "del",
                                policy_type = %policy_type,
                                "Failed to delete Redis policy type index."
                            );
                            ServiceError::internal(error)
                        })?;
                }

                Ok(deleted)
            }
        }
    }
}

#[async_trait]
impl Adapter for RedisPolicyStoreAdapter {
    async fn load_policy(&mut self, m: &mut dyn Model) -> CasbinResult<()> {
        let policies = self.list(None).await.map_err(to_casbin_error)?;
        for policy in policies {
            insert_policy_into_model(
                m,
                "p",
                policy.policy_type.as_str(),
                &policy_to_rule(&policy),
            );
        }

        Ok(())
    }

    async fn load_filtered_policy<'a>(
        &mut self,
        m: &mut dyn Model,
        f: Filter<'a>,
    ) -> CasbinResult<()> {
        let policies = self.list(None).await.map_err(to_casbin_error)?;
        for policy in policies {
            let rule = policy_to_rule(&policy);
            if matches_filter("p", &rule, &f) {
                insert_policy_into_model(m, "p", policy.policy_type.as_str(), &rule);
            }
        }

        Ok(())
    }

    async fn save_policy(&mut self, m: &mut dyn Model) -> CasbinResult<()> {
        self.clear(None).await.map_err(to_casbin_error)?;

        if let Some(ast_map) = m.get_model().get("p") {
            for (ptype, ast) in ast_map {
                for rule in ast.get_policy() {
                    let policy = policy_from_rule(ptype, rule)?;
                    self.upsert(policy).await.map_err(to_casbin_error)?;
                }
            }
        }

        Ok(())
    }

    async fn clear_policy(&mut self) -> CasbinResult<()> {
        self.clear(None).await.map_err(to_casbin_error)?;
        Ok(())
    }

    async fn add_policy(
        &mut self,
        sec: &str,
        ptype: &str,
        rule: Vec<String>,
    ) -> CasbinResult<bool> {
        if sec != "p" {
            return Ok(true);
        }

        let policy = policy_from_rule(ptype, &rule)?;
        self.upsert(policy).await.map_err(to_casbin_error)?;
        Ok(true)
    }

    async fn add_policies(
        &mut self,
        sec: &str,
        ptype: &str,
        rules: Vec<Vec<String>>,
    ) -> CasbinResult<bool> {
        let mut all_added = true;
        for rule in rules {
            if !self.add_policy(sec, ptype, rule).await? {
                all_added = false;
            }
        }
        Ok(all_added)
    }

    async fn remove_policy(
        &mut self,
        sec: &str,
        ptype: &str,
        rule: Vec<String>,
    ) -> CasbinResult<bool> {
        if sec != "p" {
            return Ok(false);
        }

        let policy = policy_from_rule(ptype, &rule)?;
        self.delete(policy.id).await.map_err(to_casbin_error)
    }

    async fn remove_policies(
        &mut self,
        sec: &str,
        ptype: &str,
        rules: Vec<Vec<String>>,
    ) -> CasbinResult<bool> {
        let mut all_removed = true;
        for rule in rules {
            if !self.remove_policy(sec, ptype, rule).await? {
                all_removed = false;
            }
        }
        Ok(all_removed)
    }

    async fn remove_filtered_policy(
        &mut self,
        sec: &str,
        ptype: &str,
        field_index: usize,
        field_values: Vec<String>,
    ) -> CasbinResult<bool> {
        if sec != "p" {
            return Ok(false);
        }

        let policies = self.list(None).await.map_err(to_casbin_error)?;
        let mut removed = false;
        for policy in policies {
            let rule = policy_to_rule(&policy);
            if matches_filtered_rule(ptype, &rule, field_index, &field_values) {
                removed |= self.delete(policy.id).await.map_err(to_casbin_error)?;
            }
        }

        Ok(removed)
    }

    fn is_filtered(&self) -> bool {
        false
    }
}

fn policy_key(namespace: &str, policy_id: &str) -> String {
    let digest = Sha256::digest(policy_id.as_bytes());
    let mut hex = String::with_capacity(digest.len() * 2);
    for byte in digest {
        use std::fmt::Write as _;
        let _ = write!(&mut hex, "{byte:02x}");
    }

    format!("{namespace}:policy:{hex}")
}

fn policy_index_key(namespace: &str, policy_type: &str) -> String {
    format!("{namespace}:policy:index:type:{policy_type}")
}

fn policy_all_index_key(namespace: &str) -> String {
    format!("{namespace}:policy:index:all")
}

fn policy_types_index_key(namespace: &str) -> String {
    format!("{namespace}:policy:index:types")
}

async fn fetch_policy(
    namespace: &str,
    connection: &mut redis::aio::ConnectionManager,
    key: &str,
) -> Result<Option<AuthorizationPolicy>, ServiceError> {
    let maybe_value: Option<String> = connection.get(key).await.map_err(|error| {
        tracing::error!(
            error = ?error,
            namespace = %namespace,
            operation = "get",
            key = %key,
            "Failed to read authorization policy from Redis."
        );
        ServiceError::internal(error)
    })?;

    let Some(value) = maybe_value else {
        return Ok(None);
    };

    let policy = serde_json::from_str::<AuthorizationPolicy>(&value).map_err(|error| {
        tracing::error!(
            error = ?error,
            namespace = %namespace,
            operation = "deserialize",
            key = %key,
            "Failed to deserialize authorization policy."
        );
        ServiceError::internal(error)
    })?;

    Ok(Some(policy))
}

async fn remove_from_indexes(
    namespace: &str,
    connection: &mut redis::aio::ConnectionManager,
    policy_key: &str,
    policy_type: &str,
) -> Result<(), ServiceError> {
    let index_key = policy_index_key(namespace, policy_type);
    let all_index_key = policy_all_index_key(namespace);

    let _: () = connection
        .srem(&index_key, policy_key)
        .await
        .map_err(|error| {
            tracing::error!(
                error = ?error,
                namespace = %namespace,
                operation = "srem",
                index_key = %index_key,
                policy_key = %policy_key,
                "Failed to remove authorization policy from Redis index."
            );
            ServiceError::internal(error)
        })?;

    let _: () = connection
        .srem(&all_index_key, policy_key)
        .await
        .map_err(|error| {
            tracing::error!(
                error = ?error,
                namespace = %namespace,
                operation = "srem",
                index_key = %all_index_key,
                policy_key = %policy_key,
                "Failed to remove authorization policy from Redis registry."
            );
            ServiceError::internal(error)
        })?;

    Ok(())
}

fn policy_to_rule(policy: &AuthorizationPolicy) -> Vec<String> {
    vec![
        policy.subject.clone(),
        policy.object.clone(),
        policy.action.clone(),
        effect_to_str(policy.effect).to_string(),
    ]
}

fn policy_from_rule(ptype: &str, rule: &[String]) -> Result<AuthorizationPolicy, casbin::Error> {
    if rule.len() < 4 {
        return Err(std::io::Error::other("invalid policy rule").into());
    }

    let effect = match rule[3].as_str() {
        "allow" => application::authorization::AuthorizationEffect::Allow,
        "deny" => application::authorization::AuthorizationEffect::Deny,
        other => {
            return Err(std::io::Error::other(format!("invalid effect: {other}")).into());
        }
    };

    Ok(AuthorizationPolicy::new(
        ptype.to_string(),
        rule[0].clone(),
        rule[1].clone(),
        rule[2].clone(),
        effect,
    ))
}

fn effect_to_str(effect: application::authorization::AuthorizationEffect) -> &'static str {
    match effect {
        application::authorization::AuthorizationEffect::Allow => "allow",
        application::authorization::AuthorizationEffect::Deny => "deny",
    }
}

fn matches_filter(sec: &str, rule: &[String], f: &Filter<'_>) -> bool {
    let filter = if sec == "p" { &f.p } else { &f.g };
    for (i, expected) in filter.iter().enumerate() {
        if !expected.is_empty() && rule.get(i).is_some_and(|actual| actual != expected) {
            return false;
        }
    }
    true
}

fn matches_filtered_rule(
    ptype: &str,
    rule: &[String],
    field_index: usize,
    field_values: &[String],
) -> bool {
    if ptype.is_empty() {
        return false;
    }

    for (i, field_value) in field_values.iter().enumerate() {
        if !field_value.is_empty()
            && rule
                .get(field_index + i)
                .is_some_and(|actual| actual != field_value)
        {
            return false;
        }
    }

    true
}

fn insert_policy_into_model(m: &mut dyn Model, sec: &str, ptype: &str, rule: &[String]) {
    if let Some(ast_map) = m.get_mut_model().get_mut(sec)
        && let Some(ast) = ast_map.get_mut(ptype)
    {
        ast.get_mut_policy().insert(rule.to_vec());
    }
}

fn to_casbin_error(error: ServiceError) -> casbin::Error {
    casbin::Error::from(std::io::Error::other(error.to_string()))
}
