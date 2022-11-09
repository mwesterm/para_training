use std::fmt::Display;

pub mod auth;
pub mod login;
pub mod tls;

#[derive(Debug, serde::Deserialize)]
pub struct LoginUserPayload {
    pub username: String,
    pub password: String,
}

impl Display for LoginUserPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "user-Name:{}, password:{}", self.username, self.password)
    }
}
