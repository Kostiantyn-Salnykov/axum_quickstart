use crate::errors::ServiceError;

pub trait PasswordHasherPort: Send + Sync {
    fn hash(&self, plaintext: &str) -> Result<String, ServiceError>;
    fn verify(&self, plaintext: &str, hash: &str) -> Result<bool, ServiceError>;
}
