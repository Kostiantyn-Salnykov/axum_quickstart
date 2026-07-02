mod values;

use crate::adapters::persistence::seaorm::entities::prelude::Users as UserEntity;
use crate::adapters::persistence::seaorm::entities::users;
use crate::adapters::persistence::search::{
    EntitySearchSpec, FieldCapabilities, FieldDef, ScalarKind, SeaOrmSearchSpec,
};
use application::search::query::SearchFilterOperator;
use application::users::search::query::UserSearchField;
use application::users::search::result::UserSearchResult;
use sea_orm::Order;

pub(crate) struct UserSearchSpec;

const TEXT_FILTER_OPS: &[SearchFilterOperator] = &[
    SearchFilterOperator::Eq,
    SearchFilterOperator::Ne,
    SearchFilterOperator::Gt,
    SearchFilterOperator::Ge,
    SearchFilterOperator::Lt,
    SearchFilterOperator::Le,
    SearchFilterOperator::Contains,
    SearchFilterOperator::In,
    SearchFilterOperator::Nin,
];
const UUID_FILTER_OPS: &[SearchFilterOperator] = &[
    SearchFilterOperator::Eq,
    SearchFilterOperator::Ne,
    SearchFilterOperator::Gt,
    SearchFilterOperator::Ge,
    SearchFilterOperator::Lt,
    SearchFilterOperator::Le,
    SearchFilterOperator::In,
    SearchFilterOperator::Nin,
];
const ENUM_FILTER_OPS: &[SearchFilterOperator] = &[
    SearchFilterOperator::Eq,
    SearchFilterOperator::Ne,
    SearchFilterOperator::In,
    SearchFilterOperator::Nin,
];
const DATE_FILTER_OPS: &[SearchFilterOperator] = &[
    SearchFilterOperator::Eq,
    SearchFilterOperator::Ne,
    SearchFilterOperator::Gt,
    SearchFilterOperator::Ge,
    SearchFilterOperator::Lt,
    SearchFilterOperator::Le,
    SearchFilterOperator::In,
    SearchFilterOperator::Nin,
];

const USER_SEARCH_FIELDS: &[FieldDef<UserSearchField, users::Column>] = &[
    FieldDef::new(
        UserSearchField::Id,
        users::Column::Id,
        ScalarKind::Uuid,
        FieldCapabilities::new(false, true, true, UUID_FILTER_OPS),
        values::parse_uuid,
    ),
    FieldDef::new(
        UserSearchField::FirstName,
        users::Column::FirstName,
        ScalarKind::Text,
        FieldCapabilities::new(true, true, true, TEXT_FILTER_OPS),
        values::parse_text,
    ),
    FieldDef::new(
        UserSearchField::LastName,
        users::Column::LastName,
        ScalarKind::Text,
        FieldCapabilities::new(true, true, true, TEXT_FILTER_OPS),
        values::parse_text,
    ),
    FieldDef::new(
        UserSearchField::Email,
        users::Column::Email,
        ScalarKind::Text,
        FieldCapabilities::new(true, true, true, TEXT_FILTER_OPS),
        values::parse_text,
    ),
    FieldDef::new(
        UserSearchField::Phone,
        users::Column::Phone,
        ScalarKind::Text,
        FieldCapabilities::new(false, true, true, TEXT_FILTER_OPS),
        values::parse_text,
    ),
    FieldDef::new(
        UserSearchField::Status,
        users::Column::Status,
        ScalarKind::PgEnum {
            db_type: "users_status",
        },
        FieldCapabilities::new(false, true, true, ENUM_FILTER_OPS),
        values::parse_status,
    ),
    FieldDef::new(
        UserSearchField::Provider,
        users::Column::Provider,
        ScalarKind::PgEnum {
            db_type: "auth_provider",
        },
        FieldCapabilities::new(false, true, true, ENUM_FILTER_OPS),
        values::parse_provider,
    ),
    FieldDef::new(
        UserSearchField::CreatedAt,
        users::Column::CreatedAt,
        ScalarKind::DateTime,
        FieldCapabilities::new(false, true, true, DATE_FILTER_OPS),
        values::parse_datetime,
    ),
    FieldDef::new(
        UserSearchField::UpdatedAt,
        users::Column::UpdatedAt,
        ScalarKind::DateTime,
        FieldCapabilities::new(false, true, true, DATE_FILTER_OPS),
        values::parse_datetime,
    ),
];

static USER_SEARCH_SPEC: EntitySearchSpec<UserEntity, UserSearchField> = EntitySearchSpec {
    fields: USER_SEARCH_FIELDS,
    tiebreaker: UserSearchField::Id,
    default_sort: &[(UserSearchField::CreatedAt, Order::Desc)],
};

impl SeaOrmSearchSpec for UserSearchSpec {
    type Entity = UserEntity;
    type Field = UserSearchField;
    type Result = UserSearchResult;

    fn spec() -> &'static EntitySearchSpec<Self::Entity, Self::Field> {
        &USER_SEARCH_SPEC
    }
}
