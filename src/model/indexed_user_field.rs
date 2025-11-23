use crate::persistence::schema::Users;

pub enum IndexedUserField {
    Email,
    Username,
    Id,
}

impl IndexedUserField {
    pub(crate) fn to_field_name(&self) -> (Users, Users) {
        match self {
            IndexedUserField::Email => (Users::Table, Users::Email),
            IndexedUserField::Username => (Users::Table, Users::Username),
            IndexedUserField::Id => (Users::Table, Users::Id),
        }
    }
}
