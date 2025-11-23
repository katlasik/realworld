use sea_query::Value;
use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use validator::ValidateEmail;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[sqlx(transparent)]
#[serde(try_from = "String", into = "String")]
pub struct Email(String);

impl Email {
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for Email {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.validate_email() {
            Ok(Email(value))
        } else {
            Err(format!("Invalid email format: {}", value))
        }
    }
}

impl TryFrom<&str> for Email {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.to_string().try_into()
    }
}

impl From<Email> for String {
    fn from(email: Email) -> String {
        email.0
    }
}

impl Display for Email {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for Email {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Email> for Value {
    fn from(e: Email) -> Self {
        Value::String(Some(Box::new(e.value().to_string())))
    }
}
