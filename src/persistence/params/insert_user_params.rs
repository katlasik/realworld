use crate::model::values::email::Email;
use crate::model::values::password_hash::PasswordHash;
use crate::model::values::username::Username;

pub struct InsertUserParams {
  pub email: Email,
  pub username: Username,
  pub password_hash: PasswordHash,
}

