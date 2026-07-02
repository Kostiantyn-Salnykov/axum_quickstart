use application::errors::ServiceError;
use domain::user::provider::AuthProvider;
use domain::user::status::UserStatus;
use sea_orm::Value;
use std::str::FromStr;
use uuid::Uuid;

pub(super) fn parse_text(value: &str) -> Result<Value, ServiceError> {
    Ok(Value::from(value.to_owned()))
}

pub(super) fn parse_uuid(value: &str) -> Result<Value, ServiceError> {
    let uuid = Uuid::parse_str(value)
        .map_err(|_| ServiceError::Validation(format!("Invalid UUID value: {value}")))?;

    Ok(Value::from(uuid))
}

pub(super) fn parse_datetime(value: &str) -> Result<Value, ServiceError> {
    let datetime = chrono::DateTime::parse_from_rfc3339(value)
        .map(|datetime| datetime.with_timezone(&chrono::Utc))
        .map_err(|_| ServiceError::Validation(format!("Invalid datetime value: {value}")))?;

    Ok(Value::from(datetime))
}

pub(super) fn parse_status(value: &str) -> Result<Value, ServiceError> {
    let status = UserStatus::from_str(value)
        .map_err(|_| ServiceError::Validation(format!("Invalid status value: {value}")))?;

    let value = match status {
        UserStatus::Unconfirmed => "unconfirmed",
        UserStatus::Confirmed => "confirmed",
        UserStatus::ForceChangePassword => "force_change_password",
        UserStatus::WaitingForDeletion => "waiting_for_deletion",
    };

    Ok(Value::from(value))
}

pub(super) fn parse_provider(value: &str) -> Result<Value, ServiceError> {
    let provider = AuthProvider::from_str(value)
        .map_err(|_| ServiceError::Validation(format!("Invalid provider value: {value}")))?;

    let value = match provider {
        AuthProvider::Google => "google",
        AuthProvider::Meta => "meta",
        AuthProvider::GitHub => "github",
    };

    Ok(Value::from(value))
}
