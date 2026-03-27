#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthProvider {
    Google,
    Meta,
    GitHub,
}

impl AuthProvider {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Google => "Google",
            Self::Meta => "Meta",
            Self::GitHub => "GitHub",
        }
    }
}
