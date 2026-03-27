#[derive(Debug, Clone)]
pub struct PasswordHash(String);

impl PasswordHash {
    pub fn from_hash(hash: String) -> Self {
        Self(hash)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn to_owned(&self) -> String {
        self.0.clone()
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}
