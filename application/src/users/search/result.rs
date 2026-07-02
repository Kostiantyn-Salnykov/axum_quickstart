pub type UserSearchPageResult = crate::search::result::SearchPageResult<UserSearchResult>;
pub type UserSearchPaginationResult = crate::search::result::SearchPaginationResult;

#[derive(Debug, Clone)]
pub struct UserSearchResult {
    pub id: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub status: Option<String>,
    pub provider: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}
