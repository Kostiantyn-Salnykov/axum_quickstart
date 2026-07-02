use super::values::{
    format_provider, format_status, parse_datetime, parse_provider, parse_status, parse_uuid,
};
use crate::adapters::persistence::seaorm::entities::prelude::Users as UserEntity;
use crate::adapters::persistence::seaorm::entities::users;
use application::errors::ServiceError;
use application::users::search::query::UserSearchField;
use sea_orm::sea_query::SimpleExpr;
use sea_orm::{ColumnTrait, EntityTrait, Order};

pub(super) fn search_column(field: UserSearchField) -> Result<users::Column, ServiceError> {
    match field {
        UserSearchField::Email => Ok(users::Column::Email),
        _ => Err(ServiceError::Validation(format!(
            "Search field `{}` is not supported for full-text searching.",
            field
        ))),
    }
}

pub(super) fn sort_column(field: UserSearchField) -> users::Column {
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

pub(super) fn cursor_order_condition(
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

pub(super) fn cursor_value_condition(
    field: UserSearchField,
    value: &str,
) -> Result<SimpleExpr, ServiceError> {
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

pub(super) fn encode_cursor_values(
    row: &<UserEntity as EntityTrait>::Model,
    sorting: &[(UserSearchField, Order)],
) -> Vec<String> {
    sorting
        .iter()
        .map(|(field, _)| match field {
            UserSearchField::Id => row.id.to_string(),
            UserSearchField::FirstName => row.first_name.to_string(),
            UserSearchField::LastName => row.last_name.to_string(),
            UserSearchField::Email => row.email.to_string(),
            UserSearchField::Phone => row.phone.clone().unwrap_or_default(),
            UserSearchField::Status => format_status(&row.status),
            UserSearchField::Provider => format_provider(&row.provider),
            UserSearchField::CreatedAt => row.created_at.to_rfc3339(),
            UserSearchField::UpdatedAt => row.updated_at.to_rfc3339(),
        })
        .collect()
}
