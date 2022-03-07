use axum::http::Extensions;
use inspirer_foundation::axum::service::ServiceProvider;
use inspirer_foundation::component::database::{Dao, DaoProvider};
use inspirer_foundation::{Error, Result};
use sea_orm::DatabaseConnection;
use lazy_static::lazy_static;
use regex::Regex;

use crate::app::model::Paginate;
use crate::app::model::content::ContentWithEntity;
use crate::app::model::{Paginated, content::Content};
use crate::app::dao::content::ContentDao;

pub struct ContentService {
    database: DatabaseConnection,
}

#[async_trait]
impl ServiceProvider for ContentService {
    async fn provide(extensions: &Extensions) -> Result<Self> {
        Ok(ContentService {
            database: extensions.get()
                .cloned()
                .ok_or(Error::ExtractServiceExtensionFailed)?,
        })
    }
}

lazy_static! {
    static ref CONTENT_NUMBER_INDEX_REGEX: Regex = Regex::new("\\d+").unwrap();
}

pub enum ContentIndex {
    Id(u64),
    Name(String),
}

impl From<String> for ContentIndex {
    fn from(s: String) -> ContentIndex {
        if CONTENT_NUMBER_INDEX_REGEX.is_match(&s) {
            ContentIndex::Id(s.parse().unwrap())
        } else {
            ContentIndex::Name(s)
        }
    }
}

impl ContentService {
    pub async fn get_content_list(&self, paginate: Paginate) -> Result<Paginated<Content>> {
        (&self.database).dao()
            .get_content_list_simple(paginate)
            .await
            .map(Paginated::map_into)
    }

    pub async fn get_content(&self, index: ContentIndex) -> Result<ContentWithEntity> {
       let res = match index {
           ContentIndex::Id(id) => (&self.database).dao().get_content_by_id(id).await?.map(Into::into),
           ContentIndex::Name(name) => (&self.database).dao().get_content_by_name(&name).await?.map(Into::into),
       };

       res.ok_or(Error::ResourceNotFound)
    }
}

pub mod private {}