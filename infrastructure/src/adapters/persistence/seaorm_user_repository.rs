use crate::adapters::persistence::seaorm::entities::prelude::Users as UserEntity;
use crate::adapters::persistence::seaorm::entities::users;
use crate::adapters::persistence::seaorm::mappers::user::{to_create_model, to_update_model};
use crate::adapters::persistence::seaorm::user_search::UserSearchSpec;
use crate::adapters::persistence::search::search_stream_with_spec;
use crate::adapters::persistence::search::search_with_spec;
use application::errors::ServiceError;
use application::search::query::SearchQuery;
use application::search::repository::SearchRepositoryPort;
use application::search::result::SearchPageResult;
use application::users::search::query::{UserSearchField, UserSearchQuery};
use application::users::search::result::{UserSearchPageResult, UserSearchResult};
use application::users::user_repository_port::UserRepositoryPort;
use async_trait::async_trait;
use domain::user::User;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, RuntimeErr,
};
use tokio::sync::mpsc::{Receiver, channel};
use uuid::Uuid;

pub struct SeaOrmUserRepositoryAdapter {
    db: DatabaseConnection,
}

impl SeaOrmUserRepositoryAdapter {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UserRepositoryPort for SeaOrmUserRepositoryAdapter {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, ServiceError> {
        let model = UserEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to find the user by id.");
                ServiceError::internal(e)
            })?;

        model.map(TryInto::try_into).transpose()
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, ServiceError> {
        let model = UserEntity::find()
            .filter(users::Column::Email.eq(email))
            .one(&self.db)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to find the user by email.");
                ServiceError::internal(e)
            })?;

        model.map(TryInto::try_into).transpose()
    }

    async fn search(&self, query: UserSearchQuery) -> Result<UserSearchPageResult, ServiceError> {
        let page = search_with_spec::<UserSearchSpec>(&self.db, query).await?;

        Ok(UserSearchPageResult {
            items: page.items.into_iter().map(UserSearchResult::from).collect(),
            pagination: page.pagination,
        })
    }

    async fn create(&self, user: &User) -> Result<User, ServiceError> {
        let active_model = to_create_model(user);
        let model = active_model.insert(&self.db).await.map_err(|e| {
            tracing::error!(
                component = "SeaOrmUserRepositoryAdapter",
                method = "create",
                error = %e,
                user_id = %user.id(),
                email = %user.email().as_str(),
                "Failed to create the user."
            );
            map_user_write_error(e)
        })?;

        model.try_into()
    }

    async fn update(&self, user: &User) -> Result<User, ServiceError> {
        let active_model = to_update_model(user);
        let model = active_model.update(&self.db).await.map_err(|e| {
            tracing::error!(error = %e, "Failed to update the user.");
            ServiceError::internal(e)
        })?;

        model.try_into()
    }
}

#[async_trait]
impl SearchRepositoryPort<UserSearchField, UserSearchResult> for SeaOrmUserRepositoryAdapter {
    async fn search(
        &self,
        query: SearchQuery<UserSearchField>,
    ) -> Result<SearchPageResult<UserSearchResult>, ServiceError> {
        UserRepositoryPort::search(self, query).await
    }

    async fn stream(
        &self,
        query: SearchQuery<UserSearchField>,
    ) -> Result<Receiver<Result<UserSearchResult, ServiceError>>, ServiceError> {
        let rows = search_stream_with_spec::<UserSearchSpec>(&self.db, query).await?;
        let (sender, receiver) = channel(32);

        tokio::spawn(async move {
            let mut rows = rows;
            while let Some(item) = rows.recv().await {
                let item = item.map(UserSearchResult::from);
                if sender.send(item).await.is_err() {
                    return;
                }
            }
        });

        Ok(receiver)
    }
}

fn map_user_write_error(error: DbErr) -> ServiceError {
    if let Some(message) = unique_violation_message(&error) {
        return ServiceError::Conflict(message);
    }

    ServiceError::internal(error)
}

fn unique_violation_message(error: &DbErr) -> Option<String> {
    match error {
        DbErr::Exec(RuntimeErr::SqlxError(error)) | DbErr::Query(RuntimeErr::SqlxError(error)) => {
            let database_error = std::ops::Deref::deref(error).as_database_error()?;

            if database_error.code().as_deref() != Some("23505") {
                return None;
            }

            let message = match database_error.constraint() {
                Some("uidx_users_email") => "User with this email already exists.",
                Some("uidx_users_phone") => "User with this phone already exists.",
                _ => "User already exists.",
            };

            Some(message.to_string())
        }
        _ => None,
    }
}
