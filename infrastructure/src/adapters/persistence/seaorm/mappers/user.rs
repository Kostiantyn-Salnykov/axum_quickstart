use crate::adapters::persistence::seaorm::entities::users::ActiveModel;
pub(crate) use crate::adapters::persistence::seaorm::entities::users::Model as UserRow;
use application::errors::ServiceError;
use chrono::{DateTime, Utc};
use domain::user::email::Email;
use domain::user::password_hash::PasswordHash;
use domain::user::phone::Phone;
use domain::user::provider::AuthProvider;
use domain::user::status::UserStatus;
use domain::user::user::User;
use sea_orm::Set;
use std::str::FromStr;

impl TryFrom<UserRow> for User {
    type Error = ServiceError;

    fn try_from(row: UserRow) -> Result<Self, Self::Error> {
        let email = map_email(&row)?;
        let phone = map_phone(&row)?;
        let status = map_status(&row)?;
        let provider = map_provider(&row)?;

        Ok(User {
            id: row.id,
            first_name: row.first_name,
            last_name: row.last_name,
            email,
            phone,
            password_hash: row.password_hash.map(PasswordHash::from),
            status,
            provider,
            created_at: row.created_at.into(),
            updated_at: row.updated_at.into(),
        })
    }
}

fn map_phone(row: &UserRow) -> Result<Option<Phone>, ServiceError> {
    row.phone
        .as_deref()
        .map(Phone::new)
        .transpose()
        .map_err(|e| {
            tracing::error!(
                error = %e,
                user_id = %row.id,
                phone = ?row.phone,
                "Failed to map user row: invalid phone"
            );
            ServiceError::internal(e)
        })
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
        email: Set(user.email.to_string()),
        phone: Set(user.phone.as_ref().map(|phone| phone.to_string())),
        password_hash: Set(user
            .password_hash
            .as_ref()
            .map(|hash| hash.as_ref().to_owned())),
        status: Set(user.status.to_string()),
        provider: Set(user.provider.as_ref().map(|provider| provider.to_string())),
        created_at: Set(now.into()),
        updated_at: Set(now.into()),
    }
}

pub(crate) fn to_update_model(user: &User, now: DateTime<Utc>) -> ActiveModel {
    ActiveModel {
        id: Set(user.id),
        first_name: Set(user.first_name.clone()),
        last_name: Set(user.last_name.clone()),
        email: Set(user.email.to_string()),
        phone: Set(user.phone.as_ref().map(|phone| phone.to_string())),
        password_hash: Set(user
            .password_hash
            .as_ref()
            .map(|hash| hash.as_ref().to_owned())),
        status: Set(user.status.to_string()),
        provider: Set(user.provider.as_ref().map(|provider| provider.to_string())),
        updated_at: Set(now.into()),
        ..Default::default()
    }
}
