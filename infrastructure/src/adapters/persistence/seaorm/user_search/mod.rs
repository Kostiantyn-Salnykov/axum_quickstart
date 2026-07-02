mod conditions;
mod cursor;
mod values;

use crate::adapters::persistence::seaorm::entities::prelude::Users as UserEntity;
use crate::adapters::persistence::search::SeaOrmSearchSpec;
use application::errors::ServiceError;
use application::search::query::SearchFilterOperator;
use application::users::search::query::UserSearchField;
use sea_orm::sea_query::SimpleExpr;
use sea_orm::{EntityTrait, Order};

pub(crate) struct UserSearchSpec;

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
        cursor::search_column(field)
    }

    fn sort_column(field: Self::Field) -> <Self::Entity as EntityTrait>::Column {
        cursor::sort_column(field)
    }

    fn filter_condition(
        field: Self::Field,
        operator: SearchFilterOperator,
        values: &[String],
    ) -> Result<SimpleExpr, ServiceError> {
        conditions::build_leaf_condition(field, operator, values)
    }

    fn cursor_order_condition(
        field: Self::Field,
        value: &str,
        order: Order,
    ) -> Result<SimpleExpr, ServiceError> {
        cursor::cursor_order_condition(field, value, order)
    }

    fn cursor_value_condition(field: Self::Field, value: &str) -> Result<SimpleExpr, ServiceError> {
        cursor::cursor_value_condition(field, value)
    }

    fn cursor_values(
        row: &<Self::Entity as EntityTrait>::Model,
        sorting: &[(Self::Field, Order)],
    ) -> Vec<String> {
        cursor::encode_cursor_values(row, sorting)
    }
}
