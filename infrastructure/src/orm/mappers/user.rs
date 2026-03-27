use crate::orm::entities::users::ActiveModel;
pub(crate) use crate::orm::entities::users::Model as UserRow;
use application::errors::ServiceError;
use chrono::{DateTime, Utc};
use domain::user::email::Email;
use domain::user::password_hash::PasswordHash;
use domain::user::provider::AuthProvider;
use domain::user::status::UserStatus;
use domain::user::user::User;
use sea_orm::Set;
use std::str::FromStr;

impl TryFrom<UserRow> for User {
    type Error = ServiceError;

    fn try_from(row: UserRow) -> Result<Self, Self::Error> {
        let email = map_email(&row)?;
        let status = map_status(&row)?;
        let provider = map_provider(&row)?;

        Ok(User {
            id: row.id,
            first_name: row.first_name,
            last_name: row.last_name,
            email,
            password_hash: row.password_hash.map(PasswordHash::from_hash),
            status,
            provider,
            created_at: row.created_at.into(),
            updated_at: row.updated_at.into(),
        })
    }
}

fn map_email(row: &UserRow) -> Result<Email, ServiceError> {
    Email::new(&row.email).map_err(|e| {
        tracing::error!(
            error = %e,
            user_id = %row.id,
            email = %row.email,
            "Failed to map user row: invalid email"
        );
        ServiceError::internal(e)
    })
}

fn map_status(row: &UserRow) -> Result<UserStatus, ServiceError> {
    UserStatus::from_str(&row.status).map_err(|_| {
        tracing::error!(
            user_id = %row.id,
            status = %row.status,
            "Failed to map user row: unknown status"
        );
        ServiceError::internal(anyhow::anyhow!(
            "Unknown user status in database: {}",
            row.status
        ))
    })
}

fn map_provider(row: &UserRow) -> Result<Option<AuthProvider>, ServiceError> {
    match row.provider.as_deref() {
        Some(raw) => AuthProvider::from_str(raw).map(Some).map_err(|_| {
            tracing::error!(
                user_id = %row.id,
                provider = %raw,
                "Failed to map user row: unknown provider"
            );
            ServiceError::internal(anyhow::anyhow!(
                "Unknown auth provider in database: {}",
                raw
            ))
        }),
        None => Ok(None),
    }
}

pub(crate) fn to_create_model(user: &User, now: DateTime<Utc>) -> ActiveModel {
    ActiveModel {
        id: Set(user.id),
        first_name: Set(user.first_name.clone()),
        last_name: Set(user.last_name.clone()),
        email: Set(user.email.to_owned()),
        password_hash: Set(user.password_hash.as_ref().map(PasswordHash::to_owned)),
        status: Set(user.status.as_str().to_owned()),
        provider: Set(user
            .provider
            .as_ref()
            .map(|provider| provider.as_str().to_owned())),
        created_at: Set(now.into()),
        updated_at: Set(now.into()),
    }
}

pub(crate) fn to_update_model(user: &User, now: DateTime<Utc>) -> ActiveModel {
    ActiveModel {
        id: Set(user.id),
        first_name: Set(user.first_name.clone()),
        last_name: Set(user.last_name.clone()),
        email: Set(user.email.to_owned()),
        password_hash: Set(user.password_hash.as_ref().map(PasswordHash::to_owned)),
        status: Set(user.status.as_str().to_owned()),
        provider: Set(user
            .provider
            .as_ref()
            .map(|provider| provider.as_str().to_owned())),
        updated_at: Set(now.into()),
        ..Default::default()
    }
}
