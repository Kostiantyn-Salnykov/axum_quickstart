use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct RefreshResult {
    pub user_id: Uuid,
    pub access_token: String,
    pub refresh_token: String,
    pub access_expires_at: DateTime<Utc>,
    pub refresh_expires_at: DateTime<Utc>,
}
