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

impl TryFrom<UserRow> for User {
    type Error = ServiceError;

    fn try_from(row: UserRow) -> Result<Self, Self::Error> {
        Ok(User {
            id: row.id,
            first_name: row.first_name,
            last_name: row.last_name,
            email: Email::new(&row.email).map_err(|e| {
                tracing::error!(
                    error = %e,
                    user_id = %row.id,
                    email = %row.email,
                    "Failed to map user row: invalid email"
                );
                ServiceError::Internal
            })?,
            password_hash: row.password_hash.map(PasswordHash::from_hash),
            status: match row.status.as_str() {
                "Unconfirmed" => UserStatus::Unconfirmed,
                "Confirmed" => UserStatus::Confirmed,
                "ResetRequired" => UserStatus::ResetRequired,
                "ForceChangePassword" => UserStatus::ForceChangePassword,
                "ExternalProvider" => UserStatus::ExternalProvider,
                other => {
                    tracing::error!(
                    user_id = %row.id,
                    status = %other,
                    "Failed to map user row: unknown status"
                    );
                    return Err(ServiceError::Internal);
                }
            },
            provider: match row.provider.as_deref() {
                Some("Google") => Some(AuthProvider::Google),
                Some("Meta") => Some(AuthProvider::Meta),
                Some("GitHub") => Some(AuthProvider::GitHub),
                Some(other) => {
                    tracing::error!(
                        user_id = %row.id,
                        provider = %other,
                        "Failed to map user row: unknown provider"
                    );
                    return Err(ServiceError::Internal);
                }
                None => None,
            },
            created_at: row.created_at.into(),
            updated_at: row.updated_at.into(),
        })
    }
}

pub(crate) fn to_new_user_active_model(user: &User, now: DateTime<Utc>) -> ActiveModel {
    ActiveModel {
        id: Set(user.id),
        first_name: Set(user.first_name.clone()),
        last_name: Set(user.last_name.clone()),
        email: Set(user.email.as_str().to_string()),
        password_hash: Set(user.password_hash.as_ref().map(PasswordHash::to_owned)),
        status: Set(user.status.as_str().to_owned()),
        provider: Set(user.provider.as_ref().map(|p| format!("{:?}", p))),
        created_at: Set(now.into()),
        updated_at: Set(now.into()),
    }
}

pub(crate) fn to_existing_user_active_model(user: &User, now: DateTime<Utc>) -> ActiveModel {
    ActiveModel {
        id: Set(user.id),
        first_name: Set(user.first_name.clone()),
        last_name: Set(user.last_name.clone()),
        email: Set(user.email.as_str().to_string()),
        password_hash: Set(user.password_hash.as_ref().map(PasswordHash::to_owned)),
        status: Set(user.status.as_str().to_owned()),
        provider: Set(user.provider.as_ref().map(|p| format!("{:?}", p))),
        updated_at: Set(now.into()),
        ..Default::default()
    }
}
