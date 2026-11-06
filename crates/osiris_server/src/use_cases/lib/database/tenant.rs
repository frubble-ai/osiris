//! Whose data key encrypts a value: a connection's user, or osiris itself.

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Tenant {
    /// Osiris's own secrets: the auth configs' client secrets.
    Osiris,
    /// A client's user: their connections' tokens and PKCE verifiers.
    User(String),
}

impl Tenant {
    pub fn user(user_id: impl Into<String>) -> Self {
        Self::User(user_id.into())
    }

    /// The key it is stored under: never clashes between the two kinds.
    pub fn key(&self) -> String {
        match self {
            Self::Osiris => "osiris".to_owned(),
            Self::User(id) => format!("user:{id}"),
        }
    }
}
