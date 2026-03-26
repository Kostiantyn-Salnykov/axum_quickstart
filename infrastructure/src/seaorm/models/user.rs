use time::OffsetDateTime;
use uuid::Uuid;

pub struct UserRow {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password_hash: Option<String>,
    pub status: String,
    pub provider: Option<String>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

pub struct InsertUser {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password_hash: Option<String>,
    pub status: String,
    pub provider: Option<String>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}
