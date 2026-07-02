use application::users::search::result::UserSearchResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct UserSearchRow {
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

impl From<UserSearchRow> for UserSearchResult {
    fn from(value: UserSearchRow) -> Self {
        Self {
            id: value.id,
            first_name: value.first_name,
            last_name: value.last_name,
            email: value.email,
            phone: value.phone,
            status: value.status,
            provider: value.provider,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
