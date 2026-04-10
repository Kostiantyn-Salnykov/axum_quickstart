use uuid::Uuid;

pub struct RegisterResult {
    pub id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub status: String,
}
