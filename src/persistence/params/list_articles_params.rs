use crate::domain::commands::list_articles_query::ListArticlesQuery;
use crate::model::limit::Limit;
use crate::model::offset::Offset;
use crate::model::values::tag_name::TagName;
use crate::model::values::user_id::UserId;
use crate::model::values::username::Username;

pub struct ListArticlesParams {
  pub(crate) tag: Option<TagName>,
  pub(crate) author: Option<Username>,
  pub(crate) favorited_by: Option<Username>,
  pub(crate) user_id: Option<UserId>,
  pub(crate) limit: Option<Limit>,
  pub(crate) offset: Option<Offset>,
}

impl ListArticlesParams {

  pub fn from_query(query: ListArticlesQuery, user_id: Option<UserId>) -> ListArticlesParams {

    ListArticlesParams {
      tag: query.tag,
      author: query.author,
      favorited_by: query.favorited_by,
      user_id,
      limit: query.limit,
      offset: query.offset,
    }


  }

}