#[derive(Debug, Clone)]
pub struct PasswordHash(String);

impl AsRef<str> for PasswordHash {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<String> for PasswordHash {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<PasswordHash> for String {
    fn from(value: PasswordHash) -> Self {
        value.0
    }
}
