mod result;
mod values;

use crate::adapters::persistence::seaorm::entities::prelude::Users as UserEntity;
use crate::adapters::persistence::seaorm::entities::users;
use application::users::search::query::UserSearchField;
pub(crate) use result::UserSearchRow;
use sea_orm::Order;

crate::define_entity_search_spec! {
    spec_type = UserSearchSpec,
    spec_const = USER_SEARCH_SPEC,
    entity = UserEntity,
    field = UserSearchField,
    result = UserSearchRow,
    tiebreaker = UserSearchField::Id,
    default_sort = [(UserSearchField::CreatedAt, Order::Desc)],
    fields = [
        {
            field = UserSearchField::Id,
            column = users::Column::Id,
            scalar = crate::adapters::persistence::search::ScalarKind::Uuid,
            searchable = false,
            sortable = true,
            projectable = true,
            parse = values::parse_uuid
        },
        {
            field = UserSearchField::FirstName,
            column = users::Column::FirstName,
            scalar = crate::adapters::persistence::search::ScalarKind::Text,
            searchable = true,
            sortable = true,
            projectable = true,
            parse = values::parse_text
        },
        {
            field = UserSearchField::LastName,
            column = users::Column::LastName,
            scalar = crate::adapters::persistence::search::ScalarKind::Text,
            searchable = true,
            sortable = true,
            projectable = true,
            parse = values::parse_text
        },
        {
            field = UserSearchField::Email,
            column = users::Column::Email,
            scalar = crate::adapters::persistence::search::ScalarKind::Text,
            searchable = true,
            sortable = true,
            projectable = true,
            parse = values::parse_text
        },
        {
            field = UserSearchField::Phone,
            column = users::Column::Phone,
            scalar = crate::adapters::persistence::search::ScalarKind::Text,
            searchable = false,
            sortable = true,
            projectable = true,
            parse = values::parse_text
        },
        {
            field = UserSearchField::Status,
            column = users::Column::Status,
            scalar = crate::adapters::persistence::search::ScalarKind::PgEnum {
                db_type: "users_status",
            },
            searchable = false,
            sortable = true,
            projectable = true,
            parse = values::parse_status
        },
        {
            field = UserSearchField::Provider,
            column = users::Column::Provider,
            scalar = crate::adapters::persistence::search::ScalarKind::PgEnum {
                db_type: "auth_provider",
            },
            searchable = false,
            sortable = true,
            projectable = true,
            parse = values::parse_provider
        },
        {
            field = UserSearchField::CreatedAt,
            column = users::Column::CreatedAt,
            scalar = crate::adapters::persistence::search::ScalarKind::DateTime,
            searchable = false,
            sortable = true,
            projectable = true,
            parse = values::parse_datetime
        },
        {
            field = UserSearchField::UpdatedAt,
            column = users::Column::UpdatedAt,
            scalar = crate::adapters::persistence::search::ScalarKind::DateTime,
            searchable = false,
            sortable = true,
            projectable = true,
            parse = values::parse_datetime
        },
    ]
}
