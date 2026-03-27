use application::results::users::RegisterUserResult;
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

impl From<RegisterUserResult> for RegisterUserResponse {
    fn from(user: RegisterUserResult) -> Self {
        Self {
            id: user.id,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            status: user.status,
        }
    }
}
