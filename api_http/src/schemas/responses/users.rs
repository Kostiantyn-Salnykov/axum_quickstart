use domain::entities::user::user::User;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct RegisterUserResponse {
    pub id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub status: String,
}

impl From<User> for RegisterUserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email.as_str().to_string(),
            first_name: user.first_name,
            last_name: user.last_name,
            status: format!("{:?}", user.status),
        }
    }
}
