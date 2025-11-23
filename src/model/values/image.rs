use sea_query::Value;
use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::fmt::{Display, Formatter};
use std::ops::Deref;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[sqlx(transparent)]
#[serde(try_from = "String", into = "String")]
pub struct Image(String);

impl Image {
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for Image {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err("Image URL cannot be empty".to_string());
        }

        if !trimmed.starts_with("http://") && !trimmed.starts_with("https://") {
            return Err("Image URL must start with http:// or https://".to_string());
        }

        if trimmed.len() > 2048 {
            return Err("Image URL cannot be longer than 2048 characters".to_string());
        }

        Ok(Image(trimmed.to_string()))
    }
}

impl TryFrom<&str> for Image {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.to_string().try_into()
    }
}

impl From<Image> for String {
    fn from(image: Image) -> String {
        image.0
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for Image {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Image> for Value {
    fn from(i: Image) -> Self {
        Value::String(Some(Box::new(i.value().to_string())))
    }
}
