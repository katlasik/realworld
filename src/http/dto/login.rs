use serde::{Deserialize, Serialize};
use crate::model::values::email::Email;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub user: LoginUser,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginUser {
    pub email: Email,
    pub password: String,
}
