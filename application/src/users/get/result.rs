use uuid::Uuid;

pub struct UserResult {
    pub id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub status: String,
    pub provider: Option<String>,
}
