use crate::persistence::schema::Articles;

pub enum IndexedArticleField {
    Slug,
    Id,
}

impl IndexedArticleField {
    pub(crate) fn to_field_name(&self) -> (Articles, Articles) {
        match self {
            IndexedArticleField::Slug => (Articles::Table, Articles::Slug),
            IndexedArticleField::Id => (Articles::Table, Articles::Id),
        }
    }
}
