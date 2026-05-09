use crate::adapters::persistence::seaorm::entities::prelude::Users as UserEntity;
use crate::adapters::persistence::seaorm::entities::users;
use crate::adapters::persistence::seaorm::mappers::user::{to_create_model, to_update_model};
use application::errors::ServiceError;
use application::users::user_repository_port::UserRepositoryPort;
use async_trait::async_trait;
use domain::user::User;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, RuntimeErr,
    SqlxError,
};
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

    async fn create(&self, user: &User) -> Result<User, ServiceError> {
        let active_model = to_create_model(user);
        let model = active_model.insert(&self.db).await.map_err(|e| {
            tracing::error!(
                component = "SeaOrmUserRepositoryAdapter",
                method = "create",
                error = %e,
                user_id = %user.id,
                email = %user.email.as_str(),
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

fn map_user_write_error(error: DbErr) -> ServiceError {
    if is_unique_violation(&error) {
        return ServiceError::Conflict("User with this email already exists.".to_string());
    }

    ServiceError::internal(error)
}

fn is_unique_violation(error: &DbErr) -> bool {
    match error {
        DbErr::Exec(RuntimeErr::SqlxError(error)) | DbErr::Query(RuntimeErr::SqlxError(error)) => {
            matches!(
                std::ops::Deref::deref(error),
                SqlxError::Database(database_error)
                    if database_error.code().as_deref() == Some("23505")
            )
        }
        _ => false,
    }
}
