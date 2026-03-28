use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct RegisterUserRequest {
    #[schema(example = "kostiantyn.salnykov@gmail.com")]
    pub email: String,
    #[schema(example = "fake123password!")]
    pub password: String,
    #[schema(example = "Kostiantyn")]
    pub first_name: Option<String>,
    #[schema(example = "Salnykov")]
    pub last_name: Option<String>,
}
