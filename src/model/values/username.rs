use std::fmt::{Display, Formatter};
use std::ops::Deref;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Username(String);

impl From<String> for Username {

  fn from(value: String) -> Self {
    Username(value)
  }
}

impl From<&str> for Username {

  fn from(value: &str) -> Self {
    value.to_string().into()
  }
}

impl From<Username> for String {
    fn from(email: Username) -> String {
        email.0
    }
}

impl Display for Username {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
      write!(f, "{}", self.0)
  }
}

impl Deref for Username {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
