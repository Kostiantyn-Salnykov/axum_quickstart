use crate::orm::entities::prelude::Users as UserEntity;
use crate::orm::entities::users;
use crate::orm::mappers::user::{UserRow, to_create_model, to_update_model};
use application::errors::ServiceError;
use application::users::user_repository::UserRepository;
use async_trait::async_trait;
use chrono::Utc;
use domain::user::user::User;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;

pub struct SeaOrmUserRepository {
    db: DatabaseConnection,
}

impl SeaOrmUserRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UserRepository for SeaOrmUserRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, ServiceError> {
        let model = UserEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to find the user by id.");
                ServiceError::internal(e)
            })?;

        model.map(|m| UserRow::from(m).try_into()).transpose()
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

        model.map(|m| UserRow::from(m).try_into()).transpose()
    }

    async fn create(&self, user: &User) -> Result<User, ServiceError> {
        let now = Utc::now();
        let active_model = to_create_model(user, now);
        let model = active_model.insert(&self.db).await.map_err(|e| {
            tracing::error!(error = %e, "Failed to create the user.");
            ServiceError::internal(e)
        })?;

        UserRow::from(model).try_into()
    }

    async fn update(&self, user: &User) -> Result<User, ServiceError> {
        let active_model = to_update_model(user, Utc::now());
        let model = active_model.update(&self.db).await.map_err(|e| {
            tracing::error!(error = %e, "Failed to update the user.");
            ServiceError::internal(e)
        })?;

        UserRow::from(model).try_into()
    }
}
