use application::auth::token_manager::{TokenAudience, TokenManager, TokenPayload};
use application::errors::ServiceError;
use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone)]
pub struct JwtTokenManager {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    access_ttl: Duration,
    refresh_ttl: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Claims {
    sub: String,
    aud: String,
    exp: i64,
    iat: i64,
}

impl JwtTokenManager {
    pub fn new(secret: impl AsRef<[u8]>, access_ttl_minutes: i64, refresh_ttl_days: i64) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_ref()),
            decoding_key: DecodingKey::from_secret(secret.as_ref()),
            access_ttl: Duration::minutes(access_ttl_minutes),
            refresh_ttl: Duration::days(refresh_ttl_days),
        }
    }

    fn issue_token(
        &self,
        user_id: Uuid,
        audience: TokenAudience,
        ttl: Duration,
    ) -> Result<(String, DateTime<Utc>), ServiceError> {
        let now = Utc::now();
        let expires_at = now + ttl;
        let claims = Claims {
            sub: user_id.to_string(),
            aud: audience.as_str().to_string(),
            exp: expires_at.timestamp(),
            iat: now.timestamp(),
        };

        let token = encode(&Header::new(Algorithm::HS256), &claims, &self.encoding_key)
            .map_err(ServiceError::internal)?;

        Ok((token, expires_at))
    }
}

impl TokenManager for JwtTokenManager {
    fn issue_access_token(&self, user_id: Uuid) -> Result<(String, DateTime<Utc>), ServiceError> {
        self.issue_token(user_id, TokenAudience::Access, self.access_ttl)
    }

    fn issue_refresh_token(&self, user_id: Uuid) -> Result<(String, DateTime<Utc>), ServiceError> {
        self.issue_token(user_id, TokenAudience::Refresh, self.refresh_ttl)
    }

    fn verify(&self, token: &str) -> Result<TokenPayload, ServiceError> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;
        validation.validate_aud = false;

        let data = decode::<Claims>(token, &self.decoding_key, &validation)
            .map_err(|_| ServiceError::InvalidCredentials)?;

        let audience = match data.claims.aud.as_str() {
            "access" => TokenAudience::Access,
            "refresh" => TokenAudience::Refresh,
            "email_confirm" => TokenAudience::EmailConfirm,
            "password_reset" => TokenAudience::PasswordReset,
            _ => return Err(ServiceError::InvalidCredentials),
        };

        let user_id =
            Uuid::parse_str(&data.claims.sub).map_err(|_| ServiceError::InvalidCredentials)?;
        let expires_at =
            DateTime::from_timestamp(data.claims.exp, 0).ok_or(ServiceError::InvalidCredentials)?;

        Ok(TokenPayload {
            user_id,
            audience,
            expires_at,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verifies_access_token_with_custom_audience_handling() {
        let manager = JwtTokenManager::new("test-secret", 15, 30);
        let user_id = Uuid::now_v7();
        let (token, _) = manager.issue_access_token(user_id).unwrap();

        let payload = manager.verify(&token).unwrap();

        assert_eq!(payload.user_id, user_id);
        assert_eq!(payload.audience, TokenAudience::Access);
    }
}
