use sea_query::Value;
use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[sqlx(transparent)]
pub struct CommentId(Uuid);

impl CommentId {
    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl From<Uuid> for CommentId {
    fn from(id: Uuid) -> Self {
        CommentId(id)
    }
}

impl From<CommentId> for Uuid {
    fn from(id: CommentId) -> Uuid {
        id.0
    }
}

impl Display for CommentId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<CommentId> for Value {
    fn from(id: CommentId) -> Self {
        Value::Uuid(Some(Box::new(id.value())))
    }
}
