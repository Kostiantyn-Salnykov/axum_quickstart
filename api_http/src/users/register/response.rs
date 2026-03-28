use application::users::register::result::RegisterUserResult;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, ToSchema)]
pub struct RegisterUserResponse {
    #[schema(example = "019d3623-2de9-72d2-bb1c-75ec4e484ee9")]
    pub id: Uuid,
    #[schema(example = "kostiantyn.salnykov@gmail.com")]
    pub email: String,
    #[schema(example = "Kostiantyn")]
    pub first_name: String,
    #[schema(example = "Salnykov")]
    pub last_name: String,
    #[schema(example = "unconfirmed")]
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
