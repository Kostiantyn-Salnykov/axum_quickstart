use application::auth::register::result::RegisterResult;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, ToSchema)]
pub struct RegisterResponse {
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

impl From<RegisterResult> for RegisterResponse {
    fn from(value: RegisterResult) -> Self {
        Self {
            id: value.id,
            email: value.email,
            first_name: value.first_name,
            last_name: value.last_name,
            status: value.status,
        }
    }
}
