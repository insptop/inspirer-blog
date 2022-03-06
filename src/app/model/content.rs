use sea_orm::prelude::DateTimeUtc;
use crate::app::model::Content as ContentModel;

#[derive(Debug, Serialize, Clone)]
pub struct Content {
    pub id: u64,
    pub title: String,
    pub keywords: String,
    pub description: String,
    pub content_name: Option<String>,
    pub is_published: bool,
    pub published_at: Option<DateTimeUtc>,
    pub modified_at: Option<DateTimeUtc>,
}

impl From<ContentModel> for Content {
    fn from(content: ContentModel) -> Self {
        Content {
            id: content.id,
            title: content.title,
            keywords: content.keywords,
            description: content.description,
            content_name: content.content_name,
            is_published: content.is_published,
            published_at: content.published_at,
            modified_at: content.modified_at
        }
    }
}