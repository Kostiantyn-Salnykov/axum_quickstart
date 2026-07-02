pub type UserSearchPageResult = crate::search::result::SearchPageResult<UserSearchResult>;
pub type UserSearchPaginationResult = crate::search::result::SearchPaginationResult;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSearchResult {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub first_name: Option<String>,
    #[serde(default)]
    pub last_name: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub phone: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub provider: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}
