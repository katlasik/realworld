use sea_query::Value;
use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::fmt::{Display, Formatter};
use std::ops::Deref;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(transparent)]
pub struct PasswordHash(String);

impl PasswordHash {
    pub fn new(hash: String) -> Self {
        PasswordHash(hash)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl From<&str> for PasswordHash {
    fn from(value: &str) -> Self {
        PasswordHash(value.to_string())
    }
}

impl From<String> for PasswordHash {
    fn from(value: String) -> Self {
        PasswordHash(value)
    }
}

impl From<PasswordHash> for String {
    fn from(hash: PasswordHash) -> String {
        hash.0
    }
}

impl Display for PasswordHash {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[REDACTED]")
    }
}

impl Deref for PasswordHash {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<PasswordHash> for Value {
    fn from(p: PasswordHash) -> Self {
        Value::String(Some(Box::new(p.value().to_string())))
    }
}
