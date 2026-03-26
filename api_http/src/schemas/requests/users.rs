use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterUserRequest {
    pub email: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}
