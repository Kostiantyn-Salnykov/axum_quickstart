use domain::user::User;
use uuid::Uuid;

pub struct RegisterResult {
    pub id: Uuid,
    pub email: String,
    pub phone: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub status: String,
}

impl From<User> for RegisterResult {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email.to_string(),
            phone: user.phone.map(|phone| phone.to_string()),
            first_name: user.first_name,
            last_name: user.last_name,
            status: user.status.to_string(),
        }
    }
}
