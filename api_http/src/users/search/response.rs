use crate::users::search::request::UserProjection;
use application::users::search::result::{
    UserSearchPageResult, UserSearchPaginationResult, UserSearchResult,
};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct UserSearchItemResponse {
    #[schema(example = "019d3623-2de9-72d2-bb1c-75ec4e484ee9", nullable = true)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[schema(example = "Kostiantyn", nullable = true)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[schema(example = "Salnykov", nullable = true)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[schema(example = "kostiantyn.salnykov@gmail.com", nullable = true)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[schema(example = "+380671234567", nullable = true)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[schema(example = "confirmed", nullable = true)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[schema(example = "google", nullable = true)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[schema(example = "2026-04-10T10:00:00Z", nullable = true)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[schema(example = "2026-04-10T10:00:00Z", nullable = true)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct UserSearchPaginationResponse {
    #[schema(example = true)]
    pub has_more: bool,
    #[schema(
        example = "2026-04-10T10:00:00Z|019d3623-2de9-72d2-bb1c-75ec4e484ee9",
        nullable = true
    )]
    pub next_cursor: Option<String>,
    #[schema(example = 0, nullable = true)]
    pub skip: Option<u64>,
    #[schema(example = 50)]
    pub limit: u64,
    #[schema(example = 1, nullable = true)]
    pub page: Option<u64>,
    #[schema(example = 50, nullable = true)]
    pub size: Option<u64>,
}

#[derive(Serialize, ToSchema)]
pub struct UserSearchResponse {
    pub items: Vec<UserSearchItemResponse>,
    pub pagination: UserSearchPaginationResponse,
}

impl UserSearchItemResponse {
    pub fn from_result(value: UserSearchResult, projection: &UserProjection) -> Self {
        let include = |field| projection.should_include(field);

        Self {
            id: include(application::users::search::query::UserSearchField::Id)
                .then(|| value.id.to_string()),
            first_name: include(application::users::search::query::UserSearchField::FirstName)
                .then_some(value.first_name),
            last_name: include(application::users::search::query::UserSearchField::LastName)
                .then_some(value.last_name),
            email: include(application::users::search::query::UserSearchField::Email)
                .then_some(value.email),
            phone: if include(application::users::search::query::UserSearchField::Phone) {
                value.phone
            } else {
                None
            },
            status: include(application::users::search::query::UserSearchField::Status)
                .then_some(value.status),
            provider: if include(application::users::search::query::UserSearchField::Provider) {
                value.provider
            } else {
                None
            },
            created_at: include(application::users::search::query::UserSearchField::CreatedAt)
                .then(|| value.created_at.to_rfc3339()),
            updated_at: include(application::users::search::query::UserSearchField::UpdatedAt)
                .then(|| value.updated_at.to_rfc3339()),
        }
    }
}

impl From<UserSearchPaginationResult> for UserSearchPaginationResponse {
    fn from(value: UserSearchPaginationResult) -> Self {
        Self {
            has_more: value.has_more,
            next_cursor: value.next_cursor.filter(|_| value.has_more),
            skip: value.skip,
            limit: value.limit,
            page: value.page,
            size: value.size,
        }
    }
}

impl UserSearchResponse {
    pub fn from_result(value: UserSearchPageResult, projection: &UserProjection) -> Self {
        Self {
            items: value
                .items
                .into_iter()
                .map(|item| UserSearchItemResponse::from_result(item, projection))
                .collect(),
            pagination: value.pagination.into(),
        }
    }
}

pub fn stream_line(
    item: UserSearchResult,
    projection: &UserProjection,
) -> Result<String, serde_json::Error> {
    serde_json::to_string(&UserSearchItemResponse::from_result(item, projection))
}
