use sea_query::Value;
use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::fmt::{Display, Formatter};
use std::ops::Deref;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[sqlx(transparent)]
#[serde(try_from = "String", into = "String")]
pub struct TagName(String);

impl TagName {
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for TagName {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err("Tag name cannot be blank".to_string());
        }

        if trimmed.len() > 255 {
            return Err("Tag name cannot be longer than 255 characters".to_string());
        }

        Ok(TagName(trimmed.to_string()))
    }
}

impl TryFrom<&str> for TagName {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.to_string().try_into()
    }
}

impl From<TagName> for String {
    fn from(tag_name: TagName) -> String {
        tag_name.0
    }
}

impl Display for TagName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for TagName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<TagName> for Value {
    fn from(t: TagName) -> Self {
        Value::String(Some(Box::new(t.value().to_string())))
    }
}

impl From<&TagName> for Value {
    fn from(t: &TagName) -> Self {
        Value::String(Some(Box::new(t.value().to_string())))
    }
}
