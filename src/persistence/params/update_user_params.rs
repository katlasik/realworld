use crate::model::values::bio::Bio;
use crate::model::values::email::Email;
use crate::model::values::image::Image;
use crate::model::values::password_hash::PasswordHash;
use crate::model::values::user_id::UserId;
use crate::model::values::username::Username;
use crate::persistence::schema::Users;

pub struct UpdateUserParams {
    pub(crate) user_id: UserId,
    pub(crate) email: Option<Email>,
    pub(crate) username: Option<Username>,
    pub(crate) password_hash: Option<PasswordHash>,
    pub(crate) bio: Option<Bio>,
    pub(crate) image: Option<Image>,
}

impl UpdateUserParams {
    pub fn as_list(&self) -> Vec<(Users, String)> {
        let mut fields = Vec::new();

        if let Some(email) = &self.email {
            fields.push((Users::Email, email.value().to_string()));
        }
        if let Some(username) = &self.username {
            fields.push((Users::Username, username.value().to_string()));
        }
        if let Some(password_hash) = &self.password_hash {
            fields.push((Users::PasswordHash, password_hash.value().to_string()));
        }
        if let Some(bio) = &self.bio {
            fields.push((Users::Bio, bio.value().to_string()));
        }
        if let Some(image) = &self.image {
            fields.push((Users::Image, image.value().to_string()));
        }

        fields
    }
}
