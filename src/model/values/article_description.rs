use sea_query::Value;
use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::fmt::{Display, Formatter};
use std::ops::Deref;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(transparent)]
#[serde(try_from = "String", into = "String")]
pub struct ArticleDescription(String);

impl ArticleDescription {
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for ArticleDescription {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err("Article description cannot be blank".to_string());
        }

        Ok(ArticleDescription(trimmed.to_string()))
    }
}

impl TryFrom<&str> for ArticleDescription {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.to_string().try_into()
    }
}

impl From<ArticleDescription> for String {
    fn from(description: ArticleDescription) -> String {
        description.0
    }
}

impl Display for ArticleDescription {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for ArticleDescription {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<ArticleDescription> for Value {
    fn from(d: ArticleDescription) -> Self {
        Value::String(Some(Box::new(d.value().to_string())))
    }
}
