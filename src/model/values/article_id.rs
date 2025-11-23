use sea_query::Value;
use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[sqlx(transparent)]
pub struct ArticleId(Uuid);

impl ArticleId {
    pub fn new() -> Self {
        ArticleId(Uuid::new_v4())
    }

    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl Default for ArticleId {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Uuid> for ArticleId {
    fn from(id: Uuid) -> Self {
        ArticleId(id)
    }
}

impl From<ArticleId> for Uuid {
    fn from(id: ArticleId) -> Uuid {
        id.0
    }
}

impl Display for ArticleId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<ArticleId> for Value {
    fn from(id: ArticleId) -> Self {
        Value::Uuid(Some(Box::new(id.value())))
    }
}
