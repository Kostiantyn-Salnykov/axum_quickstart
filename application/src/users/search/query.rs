use crate::search::query::{
    SearchFilterCombinator, SearchFilterNode, SearchFilterOperator, SearchPagination,
    SearchProjection, SearchProjectionMode, SearchQuery, SearchSearching, SearchSortDirection,
    SearchSortRule, SearchableField,
};
use std::fmt::{Display, Formatter};

pub type UserFilterCombinator = SearchFilterCombinator;
pub type UserFilterNode = SearchFilterNode<UserSearchField>;
pub type UserFilterOperator = SearchFilterOperator;
pub type UserPagination = SearchPagination;
pub type UserSearchQuery = SearchQuery<UserSearchField>;
pub type UserSearchProjection = SearchProjection<UserSearchField>;
pub type UserSearchProjectionMode = SearchProjectionMode;
pub type UserSearching = SearchSearching<UserSearchField>;
pub type UserSortDirection = SearchSortDirection;
pub type UserSortRule = SearchSortRule<UserSearchField>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UserSearchField {
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

impl UserSearchField {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Id => "id",
            Self::FirstName => "first_name",
            Self::LastName => "last_name",
            Self::Email => "email",
            Self::Phone => "phone",
            Self::Status => "status",
            Self::Provider => "provider",
            Self::CreatedAt => "created_at",
            Self::UpdatedAt => "updated_at",
        }
    }
}

impl Display for UserSearchField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl SearchableField for UserSearchField {
    fn search_fields() -> &'static [Self] {
        &[Self::FirstName, Self::LastName, Self::Email]
    }
}
