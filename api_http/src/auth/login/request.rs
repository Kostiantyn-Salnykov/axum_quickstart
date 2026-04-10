use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct LoginRequest {
    #[schema(example = "kostiantyn.salnykov@gmail.com")]
    pub email: String,
    #[schema(example = "fake123password!")]
    pub password: String,
}
