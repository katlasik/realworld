use sea_query::Value;
use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[sqlx(transparent)]
pub struct TagId(Uuid);

impl TagId {
    pub fn new() -> Self {
        TagId(Uuid::new_v4())
    }

    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl Default for TagId {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Uuid> for TagId {
    fn from(id: Uuid) -> Self {
        TagId(id)
    }
}

impl From<TagId> for Uuid {
    fn from(id: TagId) -> Uuid {
        id.0
    }
}

impl Display for TagId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<TagId> for Value {
    fn from(id: TagId) -> Self {
        Value::Uuid(Some(Box::new(id.value())))
    }
}
