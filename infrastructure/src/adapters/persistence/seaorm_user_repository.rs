use crate::adapters::persistence::seaorm::entities::prelude::Users as UserEntity;
use crate::adapters::persistence::seaorm::entities::users;
use crate::adapters::persistence::seaorm::mappers::user::{to_create_model, to_update_model};
use application::errors::ServiceError;
use application::users::user_repository_port::UserRepositoryPort;
use async_trait::async_trait;
use domain::user::User;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
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
            tracing::error!(error = %e, "Failed to create the user.");
            ServiceError::internal(e)
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
