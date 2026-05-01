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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_from_and_into_string() {
        let hash = PasswordHash::from("hashed-password".to_string());

        assert_eq!(hash.as_ref(), "hashed-password");

        let owned: String = hash.into();
        assert_eq!(owned, "hashed-password");
    }
}
