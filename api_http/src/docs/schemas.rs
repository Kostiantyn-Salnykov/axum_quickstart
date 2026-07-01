use serde::Serialize;
use utoipa::ToSchema;

#[allow(dead_code)]
#[derive(Serialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum JsendSuccessStatus {
    Success,
}

#[allow(dead_code)]
#[derive(Serialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum JsendFailStatus {
    Fail,
}

#[allow(dead_code)]
#[derive(Serialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum JsendErrorStatus {
    Error,
}

#[derive(Serialize, ToSchema)]
#[schema(title = "FailResponse", description = "Response for failed requests.")]
pub struct FailResponse {
    #[schema(example = "fail")]
    pub status: JsendFailStatus,
    pub code: u16,
    #[schema(example = "Error message.")]
    pub message: Option<String>,
    pub data: Option<()>,
}

#[derive(Serialize, ToSchema)]
#[schema(title = "ErrorResponse", description = "Response for error requests.")]
pub struct ErrorResponse {
    #[schema(example = "error")]
    pub status: JsendErrorStatus,
    #[schema(example = 500)]
    pub code: u16,
    #[schema(example = "Internal server error")]
    pub message: Option<String>,
    pub data: utoipa::TupleUnit,
}

#[derive(Serialize, ToSchema)]
pub struct HealthCheckSuccessResponse {
    #[schema(example = "success")]
    pub status: JsendSuccessStatus,
    #[schema(example = 200)]
    pub code: u16,
    #[schema(example = "Health check response from PostgreSQL and Redis.")]
    pub message: Option<String>,
    pub data: crate::health_check::check::response::HealthCheckResponse,
}

#[derive(Serialize, ToSchema)]
pub struct AuthRegisterSuccessResponse {
    #[schema(example = "success")]
    pub status: JsendSuccessStatus,
    #[schema(example = 201)]
    pub code: u16,
    #[schema(example = "User registered successfully.")]
    pub message: Option<String>,
    pub data: crate::auth::register::response::RegisterResponse,
}

#[derive(Serialize, ToSchema)]
pub struct AuthLoginSuccessResponse {
    #[schema(example = "success")]
    pub status: JsendSuccessStatus,
    #[schema(example = 200)]
    pub code: u16,
    #[schema(example = "User logged in successfully.")]
    pub message: Option<String>,
    pub data: crate::auth::login::response::LoginResponse,
}

#[derive(Serialize, ToSchema)]
pub struct AuthRefreshSuccessResponse {
    #[schema(example = "success")]
    pub status: JsendSuccessStatus,
    #[schema(example = 200)]
    pub code: u16,
    #[schema(example = "Tokens refreshed successfully.")]
    pub message: Option<String>,
    pub data: crate::auth::refresh::response::RefreshResponse,
}

#[derive(Serialize, ToSchema)]
pub struct AuthLogoutSuccessResponse {
    #[schema(example = "success")]
    pub status: JsendSuccessStatus,
    #[schema(example = 200)]
    pub code: u16,
    #[schema(example = "User logged out successfully.")]
    pub message: Option<String>,
    pub data: crate::auth::logout::response::LogoutResponse,
}

#[derive(Serialize, ToSchema)]
pub struct UsersSearchSuccessResponse {
    #[schema(example = "success")]
    pub status: JsendSuccessStatus,
    #[schema(example = 200)]
    pub code: u16,
    #[schema(example = "Users fetched successfully.")]
    pub message: Option<String>,
    pub data: crate::users::search::response::UserSearchResponse,
}
