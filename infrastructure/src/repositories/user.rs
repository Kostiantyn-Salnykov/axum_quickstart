use crate::seaorm::entities::users;
use crate::seaorm::entities::users::{ActiveModel, Entity as UserEntity};
use crate::seaorm::mappers::user::UserRow;
use async_trait::async_trait;
use chrono::Utc;
use domain::entities::user::user::User;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use service::{errors::ServiceError, ports::user::UserRepository};
use uuid::Uuid;

pub struct DbUserRepository {
    db: DatabaseConnection,
}

impl DbUserRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UserRepository for DbUserRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, ServiceError> {
        let model = UserEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| ServiceError::Internal)?;

        model.map(|m| UserRow::from(m).try_into()).transpose()
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, ServiceError> {
        let model = UserEntity::find()
            .filter(users::Column::Email.eq(email))
            .one(&self.db)
            .await
            .map_err(|e| ServiceError::Internal)?;

        model.map(|m| UserRow::from(m).try_into()).transpose()
    }

    async fn create(&self, user: &User) -> Result<User, ServiceError> {
        let now = Utc::now();
        let active = ActiveModel {
            id: Set(user.id),
            first_name: Set(user.first_name.clone()),
            last_name: Set(user.last_name.clone()),
            email: Set(user.email.as_str().to_string()),
            password_hash: Set(user.password_hash.as_ref().map(|h| h.as_str().to_string())),
            status: Set(format!("{:?}", user.status)),
            provider: Set(user.provider.as_ref().map(|p| format!("{:?}", p))),
            created_at: Set(now.into()),
            updated_at: Set(now.into()),
        };
        let model = active
            .insert(&self.db)
            .await
            .map_err(|e| ServiceError::Internal)?;

        UserRow::from(model).try_into()
    }

    async fn update(&self, user: &User) -> Result<User, ServiceError> {
        let active = ActiveModel {
            id: Set(user.id),
            first_name: Set(user.first_name.clone()),
            last_name: Set(user.last_name.clone()),
            email: Set(user.email.as_str().to_string()),
            password_hash: Set(user.password_hash.as_ref().map(|h| h.as_str().to_string())),
            status: Set(format!("{:?}", user.status)),
            provider: Set(user.provider.as_ref().map(|p| format!("{:?}", p))),
            updated_at: Set(Utc::now().into()),
            ..Default::default()
        };
        let model = active
            .update(&self.db)
            .await
            .map_err(|e| ServiceError::Internal)?;

        UserRow::from(model).try_into()
    }
}
