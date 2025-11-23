use sea_query::Value;
use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::fmt::{Display, Formatter};
use std::ops::Deref;

const MAX_BIO_LENGTH: usize = 1000;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[sqlx(transparent)]
#[serde(try_from = "String", into = "String")]
pub struct Bio(String);

impl Bio {
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for Bio {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let trimmed = value.trim();

        if trimmed.len() > MAX_BIO_LENGTH {
            return Err(format!(
                "Bio cannot be longer than {MAX_BIO_LENGTH} characters"
            ));
        }

        Ok(Bio(trimmed.to_string()))
    }
}

impl TryFrom<&str> for Bio {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.to_string().try_into()
    }
}

impl From<Bio> for String {
    fn from(bio: Bio) -> String {
        bio.0
    }
}

impl Display for Bio {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for Bio {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Bio> for Value {
    fn from(b: Bio) -> Self {
        Value::String(Some(Box::new(b.value().to_string())))
    }
}
