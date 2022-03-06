use inspirer_foundation::{component::database::Dao, Result};
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, QueryOrder, PaginatorTrait};
use futures_util::TryFutureExt;

use crate::app::entity::{prelude::*, contents::ContentType};
use crate::app::model::{Content as ContentModel, ContentEntities as ContentEntitiesModel, Paginate, Paginated};

#[async_trait]
pub trait ContentDao {
    async fn get_content_by_name(&self, name: &str) -> Result<Option<(ContentModel, Option<ContentEntitiesModel>)>>;
    async fn get_content_by_id(&self, id: u64) -> Result<Option<(ContentModel, Option<ContentEntitiesModel>)>>;
    async fn get_content_list_simple(&self, paginate: Paginate) -> Result<Paginated<ContentModel>>;
}

#[async_trait]
impl<'a, T: ConnectionTrait> ContentDao for Dao<'a, T> {
    async fn get_content_by_name(&self, name: &str) -> Result<Option<(ContentModel, Option<ContentEntitiesModel>)>> {
        Contents::find()
            .filter(contents::Column::ContentName.eq(name))
            .find_also_related(ContentEntities)
            .one(self)
            .err_into()
            .await
    }

    async fn get_content_by_id(&self, id: u64) -> Result<Option<(ContentModel, Option<content_entities::Model>)>> {
        Contents::find_by_id(id)
            .find_also_related(ContentEntities)
            .one(self)
            .err_into()
            .await
    }

    async fn get_content_list_simple(&self, paginate: Paginate) -> Result<Paginated<ContentModel>> {
        let res = Contents::find()
            .filter(contents::Column::IsPublished.eq(true))
            .filter(contents::Column::ContentType.eq(ContentType::Blog))
            .order_by_desc(contents::Column::PublishedAt)
            .order_by_desc(contents::Column::CreatedAt)
            .order_by_desc(contents::Column::Id)
            .paginate(self, paginate.per_page);
        
        Ok(Paginated {
            data: res.fetch_page(paginate.page - 1).await?,
            total: res.num_items().await?,
            last_page: res.num_pages().await?,
            page: paginate.page,
            per_page: paginate.per_page
        })
    }
}
