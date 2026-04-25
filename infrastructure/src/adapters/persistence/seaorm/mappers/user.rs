use crate::adapters::persistence::seaorm::entities::sea_orm_active_enums;
use crate::adapters::persistence::seaorm::entities::users::ActiveModel;
pub(crate) use crate::adapters::persistence::seaorm::entities::users::Model as UserRow;
use application::errors::ServiceError;
use domain::user::User;
use domain::user::email::Email;
use domain::user::password_hash::PasswordHash;
use domain::user::phone::Phone;
use domain::user::provider::AuthProvider;
use domain::user::status::UserStatus;
use sea_orm::{ActiveValue::NotSet, Set};

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
    match row.status {
        sea_orm_active_enums::UsersStatus::Unconfirmed => Ok(UserStatus::Unconfirmed),
        sea_orm_active_enums::UsersStatus::Confirmed => Ok(UserStatus::Confirmed),
        sea_orm_active_enums::UsersStatus::ForceChangePassword => {
            Ok(UserStatus::ForceChangePassword)
        }
        sea_orm_active_enums::UsersStatus::WaitingForDeletion => Ok(UserStatus::WaitingForDeletion),
    }
}

fn map_provider(row: &UserRow) -> Result<Option<AuthProvider>, ServiceError> {
    Ok(match row.provider {
        Some(sea_orm_active_enums::AuthProvider::Google) => Some(AuthProvider::Google),
        Some(sea_orm_active_enums::AuthProvider::Meta) => Some(AuthProvider::Meta),
        Some(sea_orm_active_enums::AuthProvider::Github) => Some(AuthProvider::GitHub),
        None => None,
    })
}

fn user_active_model(user: &User) -> ActiveModel {
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
        status: Set(match user.status {
            UserStatus::Unconfirmed => sea_orm_active_enums::UsersStatus::Unconfirmed,
            UserStatus::Confirmed => sea_orm_active_enums::UsersStatus::Confirmed,
            UserStatus::ForceChangePassword => {
                sea_orm_active_enums::UsersStatus::ForceChangePassword
            }
            UserStatus::WaitingForDeletion => sea_orm_active_enums::UsersStatus::WaitingForDeletion,
        }),
        provider: Set(user.provider.as_ref().map(|provider| match provider {
            AuthProvider::Google => sea_orm_active_enums::AuthProvider::Google,
            AuthProvider::Meta => sea_orm_active_enums::AuthProvider::Meta,
            AuthProvider::GitHub => sea_orm_active_enums::AuthProvider::Github,
        })),
        settings: NotSet,
        created_at: Set(user.created_at.into()),
        updated_at: Set(user.updated_at.into()),
    }
}

pub(crate) fn to_create_model(user: &User) -> ActiveModel {
    user_active_model(user)
}

pub(crate) fn to_update_model(user: &User) -> ActiveModel {
    let mut model = user_active_model(user);
    model.created_at = NotSet;
    model
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};
    use uuid::Uuid;

    fn sample_row() -> UserRow {
        UserRow {
            id: Uuid::now_v7(),
            first_name: "Ada".to_string(),
            last_name: "Lovelace".to_string(),
            email: "ada@example.com".to_string(),
            phone: Some("+380501234567".to_string()),
            password_hash: Some("hashed-password".to_string()),
            status: sea_orm_active_enums::UsersStatus::Confirmed,
            provider: Some(sea_orm_active_enums::AuthProvider::Google),
            settings: None,
            created_at: Utc::now().into(),
            updated_at: Utc::now().into(),
        }
    }

    fn sample_user() -> User {
        User {
            id: Uuid::now_v7(),
            first_name: "Ada".to_string(),
            last_name: "Lovelace".to_string(),
            email: Email::new("ada@example.com").expect("valid email"),
            phone: Some(Phone::new("+380501234567").expect("valid phone")),
            password_hash: Some(PasswordHash::from("hashed-password".to_string())),
            status: UserStatus::Confirmed,
            provider: Some(AuthProvider::Google),
            created_at: Utc::now() - Duration::days(1),
            updated_at: Utc::now(),
        }
    }

    #[test]
    fn maps_user_row_into_domain_user() {
        let row = sample_row();

        let user = User::try_from(row.clone()).expect("row should map into domain user");

        assert_eq!(user.id, row.id);
        assert_eq!(user.email.as_str(), row.email);
        assert_eq!(user.phone.as_ref().map(Phone::as_str), row.phone.as_deref());
        assert_eq!(user.status, UserStatus::Confirmed);
        assert_eq!(user.provider, Some(AuthProvider::Google));
    }

    #[test]
    fn rejects_invalid_email_from_database() {
        let mut row = sample_row();
        row.email = "not-an-email".to_string();

        let error = User::try_from(row).expect_err("invalid email should fail mapping");

        assert!(matches!(error, ServiceError::Internal { .. }));
    }

    #[test]
    fn rejects_unknown_status_from_database() {
        let row = sample_row();
        let user = User::try_from(row).expect("typed enum rows should always map");
        assert_eq!(user.status, UserStatus::Confirmed);
    }

    #[test]
    fn create_model_uses_domain_timestamps() {
        let user = sample_user();

        let model = to_create_model(&user);

        assert_eq!(model.created_at, Set(user.created_at.into()));
        assert_eq!(model.updated_at, Set(user.updated_at.into()));
    }

    #[test]
    fn update_model_does_not_overwrite_created_at() {
        let user = sample_user();

        let model = to_update_model(&user);

        assert!(matches!(model.created_at, NotSet));
        assert_eq!(model.updated_at, Set(user.updated_at.into()));
    }
}
