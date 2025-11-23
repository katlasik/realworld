use sea_query::Value;
use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::fmt::{Display, Formatter};
use std::ops::Deref;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[sqlx(transparent)]
#[serde(try_from = "String", into = "String")]
pub struct Slug(String);

impl Slug {
    pub fn from_title(title: &str) -> Self {
        let slug = title
            .to_lowercase()
            .chars()
            .map(|c| {
                if c.is_alphanumeric() {
                    c
                } else if c.is_whitespace() || c == '-' {
                    '-'
                } else {
                    ' '
                }
            })
            .collect::<String>()
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join("-");

        Slug(slug)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for Slug {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err("Slug cannot be blank".to_string());
        }

        if trimmed.len() > 255 {
            return Err("Slug cannot be longer than 255 characters".to_string());
        }

        Ok(Slug(trimmed.to_string()))
    }
}

impl TryFrom<&str> for Slug {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.to_string().try_into()
    }
}

impl From<Slug> for String {
    fn from(slug: Slug) -> String {
        slug.0
    }
}

impl Display for Slug {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for Slug {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Slug> for Value {
    fn from(s: Slug) -> Self {
        Value::String(Some(Box::new(s.value().to_string())))
    }
}

impl From<&Slug> for Value {
    fn from(s: &Slug) -> Self {
        Value::String(Some(Box::new(s.value().to_string())))
    }
}
