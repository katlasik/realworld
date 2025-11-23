use sea_query::Value;
use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::fmt::{Display, Formatter};
use std::ops::Deref;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[sqlx(transparent)]
#[serde(try_from = "String", into = "String")]
pub struct Username(String);

impl Username {
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for Username {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err("Username cannot be blank".to_string());
        }

        if trimmed.len() < 2 {
            return Err("Username must be at least 2 characters long".to_string());
        }

        if trimmed.len() > 50 {
            return Err("Username cannot be longer than 50 characters".to_string());
        }

        Ok(Username(trimmed.to_string()))
    }
}

impl TryFrom<&str> for Username {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.to_string().try_into()
    }
}

impl From<Username> for String {
    fn from(email: Username) -> String {
        email.0
    }
}

impl Display for Username {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for Username {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Username> for Value {
    fn from(u: Username) -> Self {
        Value::String(Some(Box::new(u.value().to_string())))
    }
}

impl From<&Username> for Value {
    fn from(u: &Username) -> Self {
        Value::String(Some(Box::new(u.value().to_string())))
    }
}
