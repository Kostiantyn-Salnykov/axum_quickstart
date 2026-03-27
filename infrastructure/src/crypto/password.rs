use application::{
    errors::ServiceError, ports::outbound::password_hasher::PasswordHasher as PasswordHasherPort,
};
use argon2::{
    Argon2,
    password_hash::{PasswordHasher, PasswordVerifier},
};
use password_hash::phc::PasswordHash;

pub struct ArgonPasswordHasher;

impl ArgonPasswordHasher {
    pub fn hash(password: &str) -> Result<String, argon2::password_hash::Error> {
        let hash = Argon2::default().hash_password(password.as_bytes())?;
        Ok(hash.to_string())
    }

    pub fn verify(password: &str, hash: &str) -> Result<bool, argon2::password_hash::Error> {
        let parsed = PasswordHash::new(hash)?;
        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed)
            .is_ok())
    }
}

impl PasswordHasherPort for ArgonPasswordHasher {
    fn hash(&self, plaintext: &str) -> Result<String, ServiceError> {
        Self::hash(plaintext).map_err(|_| ServiceError::Internal)
    }

    fn verify(&self, plaintext: &str, hash: &str) -> Result<bool, ServiceError> {
        Self::verify(plaintext, hash).map_err(|_| ServiceError::Internal)
    }
}
