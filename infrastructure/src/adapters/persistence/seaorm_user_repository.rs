use crate::adapters::persistence::seaorm::entities::prelude::Users as UserEntity;
use crate::adapters::persistence::seaorm::entities::sea_orm_active_enums::{
    AuthProvider as DbAuthProvider, UsersStatus as DbUserStatus,
};
use crate::adapters::persistence::seaorm::entities::users;
use crate::adapters::persistence::seaorm::mappers::user::{to_create_model, to_update_model};
use crate::adapters::persistence::search::{SeaOrmSearchSpec, search_with_spec};
use application::errors::ServiceError;
use application::search::query::{SearchFilterOperator, SearchQuery};
use application::search::repository::SearchRepositoryPort;
use application::search::result::SearchPageResult;
use application::users::search::query::{UserSearchField, UserSearchQuery};
use application::users::search::result::{
    UserSearchPageResult, UserSearchPaginationResult, UserSearchResult,
};
use application::users::user_repository_port::UserRepositoryPort;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use domain::user::User;
use domain::user::provider::AuthProvider;
use domain::user::status::UserStatus;
use sea_orm::sea_query::SimpleExpr;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, Order, QueryFilter,
    RuntimeErr,
};
use std::str::FromStr;
use uuid::Uuid;

pub struct SeaOrmUserRepositoryAdapter {
    db: DatabaseConnection,
}

impl SeaOrmUserRepositoryAdapter {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

struct UserSearchSpec;

impl SeaOrmSearchSpec for UserSearchSpec {
    type Entity = UserEntity;
    type Field = UserSearchField;

    fn default_sorting() -> Vec<(Self::Field, Order)> {
        vec![(UserSearchField::CreatedAt, Order::Desc)]
    }

    fn cursor_tiebreaker_field() -> Self::Field {
        UserSearchField::Id
    }

    fn search_column(
        field: Self::Field,
    ) -> Result<<Self::Entity as EntityTrait>::Column, ServiceError> {
        search_column(field)
    }

    fn sort_column(field: Self::Field) -> <Self::Entity as EntityTrait>::Column {
        sort_column(field)
    }

    fn filter_condition(
        field: Self::Field,
        operator: SearchFilterOperator,
        values: &[String],
    ) -> Result<SimpleExpr, ServiceError> {
        build_leaf_condition(field, operator, values)
    }

    fn cursor_order_condition(
        field: Self::Field,
        value: &str,
        order: Order,
    ) -> Result<SimpleExpr, ServiceError> {
        cursor_order_condition(field, value, order)
    }

    fn cursor_value_condition(field: Self::Field, value: &str) -> Result<SimpleExpr, ServiceError> {
        cursor_value_condition(field, value)
    }

    fn cursor_values(
        row: &<Self::Entity as EntityTrait>::Model,
        sorting: &[(Self::Field, Order)],
    ) -> Vec<String> {
        encode_cursor_values(row, sorting)
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

    async fn search(&self, query: UserSearchQuery) -> Result<UserSearchPageResult, ServiceError> {
        let page = search_with_spec::<UserSearchSpec>(&self.db, query).await?;
        let items = page
            .items
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<Vec<User>, _>>()?
            .into_iter()
            .map(UserSearchResult::from)
            .collect();

        Ok(UserSearchPageResult {
            items,
            pagination: UserSearchPaginationResult {
                has_more: page.pagination.has_more,
                next_cursor: page.pagination.next_cursor,
                skip: page.pagination.skip,
                limit: page.pagination.limit,
                page: page.pagination.page,
                size: page.pagination.size,
            },
        })
    }

    async fn create(&self, user: &User) -> Result<User, ServiceError> {
        let active_model = to_create_model(user);
        let model = active_model.insert(&self.db).await.map_err(|e| {
            tracing::error!(
                component = "SeaOrmUserRepositoryAdapter",
                method = "create",
                error = %e,
                user_id = %user.id(),
                email = %user.email().as_str(),
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

#[async_trait]
impl SearchRepositoryPort<UserSearchField, UserSearchResult> for SeaOrmUserRepositoryAdapter {
    async fn search(
        &self,
        query: SearchQuery<UserSearchField>,
    ) -> Result<SearchPageResult<UserSearchResult>, ServiceError> {
        UserRepositoryPort::search(self, query).await
    }
}

fn map_user_write_error(error: DbErr) -> ServiceError {
    if let Some(message) = unique_violation_message(&error) {
        return ServiceError::Conflict(message);
    }

    ServiceError::internal(error)
}

fn unique_violation_message(error: &DbErr) -> Option<String> {
    match error {
        DbErr::Exec(RuntimeErr::SqlxError(error)) | DbErr::Query(RuntimeErr::SqlxError(error)) => {
            let database_error = std::ops::Deref::deref(error).as_database_error()?;

            if database_error.code().as_deref() != Some("23505") {
                return None;
            }

            let message = match database_error.constraint() {
                Some("uidx_users_email") => "User with this email already exists.",
                Some("uidx_users_phone") => "User with this phone already exists.",
                _ => "User already exists.",
            };

            Some(message.to_string())
        }
        _ => None,
    }
}

fn build_leaf_condition(
    field: UserSearchField,
    operator: SearchFilterOperator,
    values: &[String],
) -> Result<SimpleExpr, ServiceError> {
    let condition = match field {
        UserSearchField::Id => build_uuid_condition(users::Column::Id, operator, values)?,
        UserSearchField::FirstName => {
            build_string_condition(users::Column::FirstName, operator, values)?
        }
        UserSearchField::LastName => {
            build_string_condition(users::Column::LastName, operator, values)?
        }
        UserSearchField::Email => build_string_condition(users::Column::Email, operator, values)?,
        UserSearchField::Phone => build_string_condition(users::Column::Phone, operator, values)?,
        UserSearchField::Status => build_status_condition(operator, values)?,
        UserSearchField::Provider => build_provider_condition(operator, values)?,
        UserSearchField::CreatedAt => {
            build_datetime_condition(users::Column::CreatedAt, operator, values)?
        }
        UserSearchField::UpdatedAt => {
            build_datetime_condition(users::Column::UpdatedAt, operator, values)?
        }
    };

    Ok(condition)
}

fn build_string_condition(
    column: users::Column,
    operator: SearchFilterOperator,
    values: &[String],
) -> Result<SimpleExpr, ServiceError> {
    let value = values
        .first()
        .cloned()
        .ok_or_else(|| ServiceError::Validation("Missing filter value.".to_string()))?;

    Ok(match operator {
        SearchFilterOperator::Gt => column.gt(value),
        SearchFilterOperator::Ge => column.gte(value),
        SearchFilterOperator::Lt => column.lt(value),
        SearchFilterOperator::Le => column.lte(value),
        SearchFilterOperator::Eq => column.eq(value),
        SearchFilterOperator::Ne => column.ne(value),
        SearchFilterOperator::Contains => column.contains(value),
        SearchFilterOperator::In => column.is_in(values.to_vec()),
        SearchFilterOperator::Nin => column.is_not_in(values.to_vec()),
    })
}

fn build_uuid_condition(
    column: users::Column,
    operator: SearchFilterOperator,
    values: &[String],
) -> Result<SimpleExpr, ServiceError> {
    let parsed = parse_uuid_values(values)?;
    let value = parsed
        .first()
        .cloned()
        .ok_or_else(|| ServiceError::Validation("Missing filter value.".to_string()))?;

    Ok(match operator {
        SearchFilterOperator::Gt => column.gt(value),
        SearchFilterOperator::Ge => column.gte(value),
        SearchFilterOperator::Lt => column.lt(value),
        SearchFilterOperator::Le => column.lte(value),
        SearchFilterOperator::Eq => column.eq(value),
        SearchFilterOperator::Ne => column.ne(value),
        SearchFilterOperator::Contains => {
            return Err(ServiceError::Validation(
                "Contains filter is only supported for text fields.".to_string(),
            ));
        }
        SearchFilterOperator::In => column.is_in(parsed),
        SearchFilterOperator::Nin => column.is_not_in(parsed),
    })
}

fn build_datetime_condition(
    column: users::Column,
    operator: SearchFilterOperator,
    values: &[String],
) -> Result<SimpleExpr, ServiceError> {
    let parsed = parse_datetime_values(values)?;
    let value = parsed
        .first()
        .cloned()
        .ok_or_else(|| ServiceError::Validation("Missing filter value.".to_string()))?;

    Ok(match operator {
        SearchFilterOperator::Gt => column.gt(value),
        SearchFilterOperator::Ge => column.gte(value),
        SearchFilterOperator::Lt => column.lt(value),
        SearchFilterOperator::Le => column.lte(value),
        SearchFilterOperator::Eq => column.eq(value),
        SearchFilterOperator::Ne => column.ne(value),
        SearchFilterOperator::Contains => {
            return Err(ServiceError::Validation(
                "Contains filter is only supported for text fields.".to_string(),
            ));
        }
        SearchFilterOperator::In => column.is_in(parsed),
        SearchFilterOperator::Nin => column.is_not_in(parsed),
    })
}

fn build_status_condition(
    operator: SearchFilterOperator,
    values: &[String],
) -> Result<SimpleExpr, ServiceError> {
    let parsed = parse_status_values(values)?;
    let value = parsed
        .first()
        .cloned()
        .ok_or_else(|| ServiceError::Validation("Missing filter value.".to_string()))?;

    match operator {
        SearchFilterOperator::Gt
        | SearchFilterOperator::Ge
        | SearchFilterOperator::Lt
        | SearchFilterOperator::Le
        | SearchFilterOperator::Contains => Err(ServiceError::Validation(
            "Ordering operators are not supported for status.".to_string(),
        )),
        SearchFilterOperator::Eq => Ok(users::Column::Status.eq(value)),
        SearchFilterOperator::Ne => Ok(users::Column::Status.ne(value)),
        SearchFilterOperator::In => Ok(users::Column::Status.is_in(parsed)),
        SearchFilterOperator::Nin => Ok(users::Column::Status.is_not_in(parsed)),
    }
}

fn build_provider_condition(
    operator: SearchFilterOperator,
    values: &[String],
) -> Result<SimpleExpr, ServiceError> {
    let parsed = parse_provider_values(values)?;
    let value = parsed
        .first()
        .cloned()
        .ok_or_else(|| ServiceError::Validation("Missing filter value.".to_string()))?;

    match operator {
        SearchFilterOperator::Gt
        | SearchFilterOperator::Ge
        | SearchFilterOperator::Lt
        | SearchFilterOperator::Le
        | SearchFilterOperator::Contains => Err(ServiceError::Validation(
            "Ordering operators are not supported for provider.".to_string(),
        )),
        SearchFilterOperator::Eq => Ok(users::Column::Provider.eq(value)),
        SearchFilterOperator::Ne => Ok(users::Column::Provider.ne(value)),
        SearchFilterOperator::In => Ok(users::Column::Provider.is_in(parsed)),
        SearchFilterOperator::Nin => Ok(users::Column::Provider.is_not_in(parsed)),
    }
}

fn search_column(field: UserSearchField) -> Result<users::Column, ServiceError> {
    match field {
        UserSearchField::Email => Ok(users::Column::Email),
        _ => Err(ServiceError::Validation(format!(
            "Search field `{}` is not supported for full-text searching.",
            field
        ))),
    }
}

fn sort_column(field: UserSearchField) -> users::Column {
    match field {
        UserSearchField::Id => users::Column::Id,
        UserSearchField::FirstName => users::Column::FirstName,
        UserSearchField::LastName => users::Column::LastName,
        UserSearchField::Email => users::Column::Email,
        UserSearchField::Phone => users::Column::Phone,
        UserSearchField::Status => users::Column::Status,
        UserSearchField::Provider => users::Column::Provider,
        UserSearchField::CreatedAt => users::Column::CreatedAt,
        UserSearchField::UpdatedAt => users::Column::UpdatedAt,
    }
}

fn cursor_order_condition(
    field: UserSearchField,
    value: &str,
    order: Order,
) -> Result<SimpleExpr, ServiceError> {
    Ok(match field {
        UserSearchField::Id => match order {
            Order::Asc => users::Column::Id.gt(parse_uuid(value)?),
            Order::Desc => users::Column::Id.lt(parse_uuid(value)?),
            _ => users::Column::Id.lt(parse_uuid(value)?),
        },
        UserSearchField::FirstName => match order {
            Order::Asc => users::Column::FirstName.gt(value.to_owned()),
            Order::Desc => users::Column::FirstName.lt(value.to_owned()),
            _ => users::Column::FirstName.lt(value.to_owned()),
        },
        UserSearchField::LastName => match order {
            Order::Asc => users::Column::LastName.gt(value.to_owned()),
            Order::Desc => users::Column::LastName.lt(value.to_owned()),
            _ => users::Column::LastName.lt(value.to_owned()),
        },
        UserSearchField::Email => match order {
            Order::Asc => users::Column::Email.gt(value.to_owned()),
            Order::Desc => users::Column::Email.lt(value.to_owned()),
            _ => users::Column::Email.lt(value.to_owned()),
        },
        UserSearchField::Phone => match order {
            Order::Asc => users::Column::Phone.gt(value.to_owned()),
            Order::Desc => users::Column::Phone.lt(value.to_owned()),
            _ => users::Column::Phone.lt(value.to_owned()),
        },
        UserSearchField::Status => match order {
            Order::Asc => users::Column::Status.gt(parse_status(value)?),
            Order::Desc => users::Column::Status.lt(parse_status(value)?),
            _ => users::Column::Status.lt(parse_status(value)?),
        },
        UserSearchField::Provider => match order {
            Order::Asc => users::Column::Provider.gt(parse_provider(value)?),
            Order::Desc => users::Column::Provider.lt(parse_provider(value)?),
            _ => users::Column::Provider.lt(parse_provider(value)?),
        },
        UserSearchField::CreatedAt => match order {
            Order::Asc => users::Column::CreatedAt.gt(parse_datetime(value)?),
            Order::Desc => users::Column::CreatedAt.lt(parse_datetime(value)?),
            _ => users::Column::CreatedAt.lt(parse_datetime(value)?),
        },
        UserSearchField::UpdatedAt => match order {
            Order::Asc => users::Column::UpdatedAt.gt(parse_datetime(value)?),
            Order::Desc => users::Column::UpdatedAt.lt(parse_datetime(value)?),
            _ => users::Column::UpdatedAt.lt(parse_datetime(value)?),
        },
    })
}

fn cursor_value_condition(field: UserSearchField, value: &str) -> Result<SimpleExpr, ServiceError> {
    Ok(match field {
        UserSearchField::Id => users::Column::Id.eq(parse_uuid(value)?),
        UserSearchField::FirstName => users::Column::FirstName.eq(value.to_owned()),
        UserSearchField::LastName => users::Column::LastName.eq(value.to_owned()),
        UserSearchField::Email => users::Column::Email.eq(value.to_owned()),
        UserSearchField::Phone => users::Column::Phone.eq(value.to_owned()),
        UserSearchField::Status => users::Column::Status.eq(parse_status(value)?),
        UserSearchField::Provider => users::Column::Provider.eq(parse_provider(value)?),
        UserSearchField::CreatedAt => users::Column::CreatedAt.eq(parse_datetime(value)?),
        UserSearchField::UpdatedAt => users::Column::UpdatedAt.eq(parse_datetime(value)?),
    })
}

fn encode_cursor_values(row: &users::Model, sorting: &[(UserSearchField, Order)]) -> Vec<String> {
    sorting
        .iter()
        .map(|(field, _)| match field {
            UserSearchField::Id => row.id.to_string(),
            UserSearchField::FirstName => row.first_name.to_string(),
            UserSearchField::LastName => row.last_name.to_string(),
            UserSearchField::Email => row.email.to_string(),
            UserSearchField::Phone => row.phone.clone().unwrap_or_default(),
            UserSearchField::Status => match &row.status {
                DbUserStatus::Unconfirmed => "unconfirmed".to_string(),
                DbUserStatus::Confirmed => "confirmed".to_string(),
                DbUserStatus::ForceChangePassword => "force_change_password".to_string(),
                DbUserStatus::WaitingForDeletion => "waiting_for_deletion".to_string(),
            },
            UserSearchField::Provider => row
                .provider
                .as_ref()
                .map(|provider| match provider {
                    DbAuthProvider::Google => "google".to_string(),
                    DbAuthProvider::Meta => "meta".to_string(),
                    DbAuthProvider::Github => "github".to_string(),
                })
                .unwrap_or_default(),
            UserSearchField::CreatedAt => row.created_at.to_rfc3339(),
            UserSearchField::UpdatedAt => row.updated_at.to_rfc3339(),
        })
        .collect()
}

fn parse_uuid(value: &str) -> Result<Uuid, ServiceError> {
    Uuid::parse_str(value)
        .map_err(|_| ServiceError::Validation(format!("Invalid UUID value: {value}")))
}

fn parse_datetime(value: &str) -> Result<DateTime<Utc>, ServiceError> {
    DateTime::parse_from_rfc3339(value)
        .map(|datetime| datetime.with_timezone(&Utc))
        .map_err(|_| ServiceError::Validation(format!("Invalid datetime value: {value}")))
}

fn parse_status(value: &str) -> Result<DbUserStatus, ServiceError> {
    let status = UserStatus::from_str(value)
        .map_err(|_| ServiceError::Validation(format!("Invalid status value: {value}")))?;

    Ok(match status {
        UserStatus::Unconfirmed => DbUserStatus::Unconfirmed,
        UserStatus::Confirmed => DbUserStatus::Confirmed,
        UserStatus::ForceChangePassword => DbUserStatus::ForceChangePassword,
        UserStatus::WaitingForDeletion => DbUserStatus::WaitingForDeletion,
    })
}

fn parse_provider(value: &str) -> Result<DbAuthProvider, ServiceError> {
    let provider = AuthProvider::from_str(value)
        .map_err(|_| ServiceError::Validation(format!("Invalid provider value: {value}")))?;

    Ok(match provider {
        AuthProvider::Google => DbAuthProvider::Google,
        AuthProvider::Meta => DbAuthProvider::Meta,
        AuthProvider::GitHub => DbAuthProvider::Github,
    })
}

fn parse_uuid_values(values: &[String]) -> Result<Vec<Uuid>, ServiceError> {
    values.iter().map(|value| parse_uuid(value)).collect()
}

fn parse_datetime_values(values: &[String]) -> Result<Vec<DateTime<Utc>>, ServiceError> {
    values.iter().map(|value| parse_datetime(value)).collect()
}

fn parse_status_values(values: &[String]) -> Result<Vec<DbUserStatus>, ServiceError> {
    values.iter().map(|value| parse_status(value)).collect()
}

fn parse_provider_values(values: &[String]) -> Result<Vec<DbAuthProvider>, ServiceError> {
    values.iter().map(|value| parse_provider(value)).collect()
}
