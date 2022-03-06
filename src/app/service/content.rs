use axum::http::Extensions;
use inspirer_foundation::axum::service::ServiceProvider;
use inspirer_foundation::component::database::Dao;
use inspirer_foundation::{Error, Result};
use sea_orm::DatabaseConnection;

use crate::app::model::Paginate;
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

impl ContentService {
    pub async fn get_content_list(&self, paginate: Paginate) -> Result<Paginated<Content>> {
        Dao(&self.database)
            .get_content_list_simple(paginate)
            .await
            .map(Paginated::map_into)
    }
}

pub mod private {}