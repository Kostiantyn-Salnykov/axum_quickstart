use super::values::{parse_datetime, parse_provider, parse_status, parse_uuid};
use crate::adapters::persistence::seaorm::entities::users;
use application::errors::ServiceError;
use application::search::query::SearchFilterOperator;
use application::users::search::query::UserSearchField;
use sea_orm::ColumnTrait;
use sea_orm::sea_query::SimpleExpr;

pub(super) fn build_leaf_condition(
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

fn parse_uuid_values(values: &[String]) -> Result<Vec<uuid::Uuid>, ServiceError> {
    values.iter().map(|value| parse_uuid(value)).collect()
}

fn parse_datetime_values(
    values: &[String],
) -> Result<Vec<chrono::DateTime<chrono::Utc>>, ServiceError> {
    values.iter().map(|value| parse_datetime(value)).collect()
}

fn parse_status_values(
    values: &[String],
) -> Result<
    Vec<crate::adapters::persistence::seaorm::entities::sea_orm_active_enums::UsersStatus>,
    ServiceError,
> {
    values.iter().map(|value| parse_status(value)).collect()
}

fn parse_provider_values(
    values: &[String],
) -> Result<
    Vec<crate::adapters::persistence::seaorm::entities::sea_orm_active_enums::AuthProvider>,
    ServiceError,
> {
    values.iter().map(|value| parse_provider(value)).collect()
}
