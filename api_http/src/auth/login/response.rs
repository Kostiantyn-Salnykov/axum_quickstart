use application::auth::login::result::LoginResult;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, ToSchema)]
pub struct LoginResponse {
    #[schema(example = "019d3623-2de9-72d2-bb1c-75ec4e484ee9")]
    pub user_id: Uuid,
    pub access_token: String,
    pub refresh_token: String,
    #[schema(example = "Bearer")]
    pub token_type: String,
    #[schema(example = "2026-04-10T10:00:00Z")]
    pub access_expires_at: String,
    #[schema(example = "2026-05-10T10:00:00Z")]
    pub refresh_expires_at: String,
}

impl From<LoginResult> for LoginResponse {
    fn from(value: LoginResult) -> Self {
        Self {
            user_id: value.user_id,
            access_token: value.access_token,
            refresh_token: value.refresh_token,
            token_type: "Bearer".to_string(),
            access_expires_at: value.access_expires_at.to_rfc3339(),
            refresh_expires_at: value.refresh_expires_at.to_rfc3339(),
        }
    }
}
