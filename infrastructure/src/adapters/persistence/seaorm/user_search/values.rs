use crate::adapters::persistence::seaorm::entities::sea_orm_active_enums::{
    AuthProvider as DbAuthProvider, UsersStatus as DbUserStatus,
};
use application::errors::ServiceError;
use domain::user::provider::AuthProvider;
use domain::user::status::UserStatus;
use std::str::FromStr;
use uuid::Uuid;

pub(super) fn parse_uuid(value: &str) -> Result<Uuid, ServiceError> {
    Uuid::parse_str(value)
        .map_err(|_| ServiceError::Validation(format!("Invalid UUID value: {value}")))
}

pub(super) fn parse_datetime(value: &str) -> Result<chrono::DateTime<chrono::Utc>, ServiceError> {
    chrono::DateTime::parse_from_rfc3339(value)
        .map(|datetime| datetime.with_timezone(&chrono::Utc))
        .map_err(|_| ServiceError::Validation(format!("Invalid datetime value: {value}")))
}

pub(super) fn parse_status(value: &str) -> Result<DbUserStatus, ServiceError> {
    let status = UserStatus::from_str(value)
        .map_err(|_| ServiceError::Validation(format!("Invalid status value: {value}")))?;

    Ok(match status {
        UserStatus::Unconfirmed => DbUserStatus::Unconfirmed,
        UserStatus::Confirmed => DbUserStatus::Confirmed,
        UserStatus::ForceChangePassword => DbUserStatus::ForceChangePassword,
        UserStatus::WaitingForDeletion => DbUserStatus::WaitingForDeletion,
    })
}

pub(super) fn parse_provider(value: &str) -> Result<DbAuthProvider, ServiceError> {
    let provider = AuthProvider::from_str(value)
        .map_err(|_| ServiceError::Validation(format!("Invalid provider value: {value}")))?;

    Ok(match provider {
        AuthProvider::Google => DbAuthProvider::Google,
        AuthProvider::Meta => DbAuthProvider::Meta,
        AuthProvider::GitHub => DbAuthProvider::Github,
    })
}

pub(super) fn format_status(value: &DbUserStatus) -> String {
    match value {
        DbUserStatus::Unconfirmed => "unconfirmed".to_string(),
        DbUserStatus::Confirmed => "confirmed".to_string(),
        DbUserStatus::ForceChangePassword => "force_change_password".to_string(),
        DbUserStatus::WaitingForDeletion => "waiting_for_deletion".to_string(),
    }
}

pub(super) fn format_provider(value: &Option<DbAuthProvider>) -> String {
    value
        .as_ref()
        .map(|provider| match provider {
            DbAuthProvider::Google => "google".to_string(),
            DbAuthProvider::Meta => "meta".to_string(),
            DbAuthProvider::Github => "github".to_string(),
        })
        .unwrap_or_default()
}
