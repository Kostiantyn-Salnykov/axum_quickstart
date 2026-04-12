use application::users::get::result::UserResult;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, ToSchema)]
pub struct UserResponse {
    #[schema(example = "019d3623-2de9-72d2-bb1c-75ec4e484ee9")]
    pub id: Uuid,
    #[schema(example = "kostiantyn.salnykov@gmail.com")]
    pub email: String,
    #[schema(example = "+380671234567", nullable = true)]
    pub phone: Option<String>,
    #[schema(example = "Kostiantyn")]
    pub first_name: String,
    #[schema(example = "Salnykov")]
    pub last_name: String,
    #[schema(example = "confirmed")]
    pub status: String,
    #[schema(example = "google", nullable = true)]
    pub provider: Option<String>,
}

impl From<UserResult> for UserResponse {
    fn from(value: UserResult) -> Self {
        Self {
            id: value.id,
            email: value.email,
            phone: value.phone,
            first_name: value.first_name,
            last_name: value.last_name,
            status: value.status,
            provider: value.provider,
        }
    }
}
