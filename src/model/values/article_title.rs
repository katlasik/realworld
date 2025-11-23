use sea_query::Value;
use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::fmt::{Display, Formatter};
use std::ops::Deref;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(transparent)]
#[serde(try_from = "String", into = "String")]
pub struct ArticleTitle(String);

impl ArticleTitle {
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for ArticleTitle {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err("Article title cannot be blank".to_string());
        }

        if trimmed.len() > 255 {
            return Err("Article title cannot be longer than 255 characters".to_string());
        }

        Ok(ArticleTitle(trimmed.to_string()))
    }
}

impl TryFrom<&str> for ArticleTitle {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.to_string().try_into()
    }
}

impl From<ArticleTitle> for String {
    fn from(title: ArticleTitle) -> String {
        title.0
    }
}

impl Display for ArticleTitle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for ArticleTitle {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<ArticleTitle> for Value {
    fn from(t: ArticleTitle) -> Self {
        Value::String(Some(Box::new(t.value().to_string())))
    }
}
