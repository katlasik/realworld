use sea_query::Iden;

#[allow(dead_code)]
#[derive(Iden)]
pub enum Users {
    Table,
    Id,
    Username,
    Email,
    PasswordHash,
    Bio,
    Image,
    CreatedAt,
    UpdatedAt,
}

#[allow(dead_code)]
#[derive(Iden, Clone)]
pub enum Articles {
    Table,
    Id,
    Slug,
    Title,
    Description,
    Body,
    AuthorId,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
pub enum Tags {
    Table,
    Id,
    Name,
    CreatedAt,
}

#[allow(dead_code)]
#[derive(Iden)]
pub enum ArticleTags {
    Table,
    ArticleId,
    TagId,
}

#[allow(dead_code)]
#[derive(Iden)]
pub enum Comments {
    Table,
    Id,
    Body,
    ArticleId,
    AuthorId,
    CreatedAt,
    UpdatedAt,
}

#[allow(dead_code)]
#[derive(Iden)]
pub enum UserFollows {
    Table,
    FollowerId,
    FolloweeId,
}

#[allow(dead_code)]
#[derive(Iden)]
pub enum ArticleFavorites {
    Table,
    UserId,
    ArticleId,
}

#[allow(dead_code)]
#[derive(Iden)]
pub enum ArticleView {
    Table,
    Id,
    Slug,
    Title,
    Description,
    CreatedAt,
    UpdatedAt,
    Body,
    AuthorId,
    AuthorUsername,
    AuthorBio,
    AuthorImage,
    FavoritesCount,
    TagList,
    Following,
    Favorited,
}

#[allow(dead_code)]
#[derive(Iden)]
pub enum CommentView {
    Table,
    Id,
    Body,
    CreatedAt,
    UpdatedAt,
    AuthorUsername,
    AuthorBio,
    AuthorImage,
    Following,
}
