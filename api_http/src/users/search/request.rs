use crate::errors::AppError;
use application::users::search::query as app;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct UserSearchRequest {
    pub searching: Option<UserSearchingRequest>,
    pub projection: Option<UserProjectionRequest>,
    pub filtration: Option<UserFilterNodeRequest>,
    pub sorting: Option<Vec<UserSortRuleRequest>>,
    pub pagination: Option<UserPaginationRequest>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct UserSearchingRequest {
    #[schema(example = "kostiantyn")]
    pub value: String,
    pub fields: Option<Vec<UserSearchFieldRequest>>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct UserProjectionRequest {
    pub mode: UserProjectionMode,
    pub fields: Vec<UserFilterFieldRequest>,
}

#[derive(Debug, Clone, Copy, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum UserProjectionMode {
    Show,
    Hide,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum UserFilterNodeRequest {
    Group {
        combinator: UserFilterCombinatorRequest,
        #[schema(no_recursion)]
        items: Vec<UserFilterNodeRequest>,
    },
    Condition {
        field: UserFilterFieldRequest,
        operator: UserFilterOperatorRequest,
        values: Vec<String>,
    },
}

#[derive(Debug, Clone, Copy, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum UserFilterCombinatorRequest {
    And,
    Or,
}

#[derive(Debug, Clone, Copy, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum UserFilterOperatorRequest {
    Gt,
    Ge,
    Lt,
    Le,
    Eq,
    Ne,
    Contains,
    In,
    Nin,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct UserSortRuleRequest {
    pub field: UserFilterFieldRequest,
    pub direction: UserSortDirectionRequest,
}

#[derive(Debug, Clone, Copy, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum UserSortDirectionRequest {
    Asc,
    Desc,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum UserPaginationRequest {
    SkipLimit { skip: u64, limit: u64 },
    PageSize { page: u64, size: u64 },
    Cursor { cursor: Option<String>, limit: u64 },
}

#[derive(Debug, Clone, Copy, Deserialize, ToSchema, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum UserSearchFieldRequest {
    FirstName,
    LastName,
    Email,
}

#[derive(Debug, Clone, Copy, Deserialize, ToSchema, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum UserFilterFieldRequest {
    Id,
    FirstName,
    LastName,
    Email,
    Phone,
    Status,
    Provider,
    CreatedAt,
    UpdatedAt,
}

#[derive(Debug, Clone)]
pub struct UserProjection {
    pub mode: UserProjectionMode,
    pub fields: Vec<app::UserSearchField>,
}

impl UserProjection {
    pub fn show_all() -> Self {
        Self {
            mode: UserProjectionMode::Show,
            fields: vec![],
        }
    }

    pub fn should_include(&self, field: app::UserSearchField) -> bool {
        match self.mode {
            UserProjectionMode::Show => self.fields.is_empty() || self.fields.contains(&field),
            UserProjectionMode::Hide => !self.fields.contains(&field),
        }
    }
}

impl UserSearchRequest {
    pub fn into_query(self) -> Result<(app::UserSearchQuery, UserProjection), AppError> {
        let searching = self.searching.map(|searching| app::UserSearching {
            value: searching.value,
            fields: searching
                .fields
                .unwrap_or_else(default_search_fields)
                .into_iter()
                .map(Into::into)
                .collect(),
        });
        let filtration = self
            .filtration
            .map(TryInto::try_into)
            .transpose()
            .map_err(AppError::Validation)?;
        let sorting = self
            .sorting
            .unwrap_or_default()
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<Vec<_>, _>>()
            .map_err(AppError::Validation)?;
        let projection_query =
            self.projection
                .as_ref()
                .map(|projection| app::UserSearchProjection {
                    mode: projection.mode.into(),
                    fields: projection.fields.iter().copied().map(Into::into).collect(),
                });
        let pagination = self
            .pagination
            .map(TryInto::try_into)
            .transpose()
            .map_err(AppError::Validation)?
            .unwrap_or(app::UserPagination::PageSize { page: 1, size: 50 });
        let projection = self
            .projection
            .map(Into::into)
            .unwrap_or_else(UserProjection::show_all);

        Ok((
            app::UserSearchQuery {
                searching,
                filtration,
                sorting,
                projection: projection_query,
                pagination,
            },
            projection,
        ))
    }
}

impl From<UserProjectionRequest> for UserProjection {
    fn from(value: UserProjectionRequest) -> Self {
        Self {
            mode: value.mode,
            fields: value.fields.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<UserProjectionMode> for app::UserSearchProjectionMode {
    fn from(value: UserProjectionMode) -> Self {
        match value {
            UserProjectionMode::Show => Self::Show,
            UserProjectionMode::Hide => Self::Hide,
        }
    }
}

impl TryFrom<UserFilterNodeRequest> for app::UserFilterNode {
    type Error = String;

    fn try_from(value: UserFilterNodeRequest) -> Result<Self, Self::Error> {
        match value {
            UserFilterNodeRequest::Group { combinator, items } => {
                let items = items
                    .into_iter()
                    .map(TryInto::try_into)
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(Self::Group {
                    combinator: combinator.into(),
                    items,
                })
            }
            UserFilterNodeRequest::Condition {
                field,
                operator,
                values,
            } => {
                if values.is_empty() {
                    return Err(format!("Filter `{:?}` requires at least one value.", field));
                }

                if !matches!(
                    operator,
                    UserFilterOperatorRequest::In
                        | UserFilterOperatorRequest::Nin
                        | UserFilterOperatorRequest::Contains
                ) && values.len() != 1
                {
                    return Err(format!("Filter `{:?}` requires exactly one value.", field));
                }

                Ok(Self::Condition {
                    field: field.into(),
                    operator: operator.into(),
                    values,
                })
            }
        }
    }
}

impl From<UserFilterCombinatorRequest> for app::UserFilterCombinator {
    fn from(value: UserFilterCombinatorRequest) -> Self {
        match value {
            UserFilterCombinatorRequest::And => Self::And,
            UserFilterCombinatorRequest::Or => Self::Or,
        }
    }
}

impl From<UserFilterOperatorRequest> for app::UserFilterOperator {
    fn from(value: UserFilterOperatorRequest) -> Self {
        match value {
            UserFilterOperatorRequest::Gt => Self::Gt,
            UserFilterOperatorRequest::Ge => Self::Ge,
            UserFilterOperatorRequest::Lt => Self::Lt,
            UserFilterOperatorRequest::Le => Self::Le,
            UserFilterOperatorRequest::Eq => Self::Eq,
            UserFilterOperatorRequest::Ne => Self::Ne,
            UserFilterOperatorRequest::Contains => Self::Contains,
            UserFilterOperatorRequest::In => Self::In,
            UserFilterOperatorRequest::Nin => Self::Nin,
        }
    }
}

impl TryFrom<UserSortRuleRequest> for app::UserSortRule {
    type Error = String;

    fn try_from(value: UserSortRuleRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            field: value.field.into(),
            direction: value.direction.into(),
        })
    }
}

impl From<UserSortDirectionRequest> for app::UserSortDirection {
    fn from(value: UserSortDirectionRequest) -> Self {
        match value {
            UserSortDirectionRequest::Asc => Self::Asc,
            UserSortDirectionRequest::Desc => Self::Desc,
        }
    }
}

impl TryFrom<UserPaginationRequest> for app::UserPagination {
    type Error = String;

    fn try_from(value: UserPaginationRequest) -> Result<Self, Self::Error> {
        Ok(match value {
            UserPaginationRequest::SkipLimit { skip, limit } => Self::SkipLimit { skip, limit },
            UserPaginationRequest::PageSize { page, size } => Self::PageSize { page, size },
            UserPaginationRequest::Cursor { cursor, limit } => Self::Cursor { cursor, limit },
        })
    }
}

impl From<UserSearchFieldRequest> for app::UserSearchField {
    fn from(value: UserSearchFieldRequest) -> Self {
        match value {
            UserSearchFieldRequest::FirstName => Self::FirstName,
            UserSearchFieldRequest::LastName => Self::LastName,
            UserSearchFieldRequest::Email => Self::Email,
        }
    }
}

fn default_search_fields() -> Vec<UserSearchFieldRequest> {
    vec![
        UserSearchFieldRequest::FirstName,
        UserSearchFieldRequest::LastName,
        UserSearchFieldRequest::Email,
    ]
}

impl From<UserFilterFieldRequest> for app::UserSearchField {
    fn from(value: UserFilterFieldRequest) -> Self {
        match value {
            UserFilterFieldRequest::Id => Self::Id,
            UserFilterFieldRequest::FirstName => Self::FirstName,
            UserFilterFieldRequest::LastName => Self::LastName,
            UserFilterFieldRequest::Email => Self::Email,
            UserFilterFieldRequest::Phone => Self::Phone,
            UserFilterFieldRequest::Status => Self::Status,
            UserFilterFieldRequest::Provider => Self::Provider,
            UserFilterFieldRequest::CreatedAt => Self::CreatedAt,
            UserFilterFieldRequest::UpdatedAt => Self::UpdatedAt,
        }
    }
}
