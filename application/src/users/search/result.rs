pub type UserSearchPageResult = crate::search::result::SearchPageResult<UserSearchResult>;
pub type UserSearchPaginationResult = crate::search::result::SearchPaginationResult;

use chrono::{DateTime, Utc};
use domain::user::User;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct UserSearchResult {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub status: String,
    pub provider: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<User> for UserSearchResult {
    fn from(user: User) -> Self {
        Self {
            id: user.id(),
            first_name: user.first_name().to_string(),
            last_name: user.last_name().to_string(),
            email: user.email().to_string(),
            phone: user.phone().map(|phone| phone.to_string()),
            status: user.status().to_string(),
            provider: user.provider().map(|provider| provider.to_string()),
            created_at: user.created_at(),
            updated_at: user.updated_at(),
        }
    }
}
