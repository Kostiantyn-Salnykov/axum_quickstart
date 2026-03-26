use crate::seaorm::entities::users::Model as UserRow;
use chrono::Utc;
use domain::entities::user::email::Email;
use domain::entities::user::password_hash::PasswordHash;
use domain::entities::user::provider::AuthProvider;
use domain::entities::user::status::UserStatus;
use domain::entities::user::user::User;
use service::errors::ServiceError;

impl TryFrom<UserRow> for User {
    type Error = ServiceError;

    fn try_from(row: UserRow) -> Result<Self, Self::Error> {
        Ok(User {
            id: row.id,
            first_name: row.first_name,
            last_name: row.last_name,
            email: Email::new(&row.email)
                .map_err(|e| ServiceError::Infrastructure(e.to_string()))?,
            password_hash: row.password_hash.map(PasswordHash::from_hash),
            status: match row.status.as_str() {
                "Unconfirmed" => UserStatus::Unconfirmed,
                "Confirmed" => UserStatus::Confirmed,
                "ResetRequired" => UserStatus::ResetRequired,
                "ForceChangePassword" => UserStatus::ForceChangePassword,
                "ExternalProvider" => UserStatus::ExternalProvider,
                other => {
                    return Err(ServiceError::Infrastructure(format!(
                        "unknown status: {other}"
                    )));
                }
            },
            provider: match row.provider.as_deref() {
                Some("Google") => Some(AuthProvider::Google),
                Some("Meta") => Some(AuthProvider::Meta),
                Some("GitHub") => Some(AuthProvider::GitHub),
                Some(other) => {
                    return Err(ServiceError::Infrastructure(format!(
                        "unknown provider: {other}"
                    )));
                }
                None => None,
            },
            created_at: row.created_at.with_timezone(&Utc),
            updated_at: row.updated_at.with_timezone(&Utc),
        })
    }
}
