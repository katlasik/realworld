use serde::{Deserialize, Serialize};
use crate::model::values::email::Email;
use crate::model::values::username::Username;

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub user: RegisterUser,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterUser {
    pub username: Username,
    pub email: Email,
    pub password: String,
}
