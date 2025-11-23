use sea_query::Value;
use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::fmt::{Display, Formatter};
use std::ops::Deref;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(transparent)]
#[serde(try_from = "String", into = "String")]
pub struct ArticleBody(String);

impl ArticleBody {
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for ArticleBody {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err("Article body cannot be blank".to_string());
        }

        Ok(ArticleBody(trimmed.to_string()))
    }
}

impl TryFrom<&str> for ArticleBody {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.to_string().try_into()
    }
}

impl From<ArticleBody> for String {
    fn from(body: ArticleBody) -> String {
        body.0
    }
}

impl Display for ArticleBody {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for ArticleBody {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<ArticleBody> for Value {
    fn from(b: ArticleBody) -> Self {
        Value::String(Some(Box::new(b.value().to_string())))
    }
}
