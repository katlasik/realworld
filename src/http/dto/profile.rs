use serde::{Deserialize, Serialize};
use crate::model::values::username::Username;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileResponse {
    pub profile: Profile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub username: Username,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub following: bool,
}
