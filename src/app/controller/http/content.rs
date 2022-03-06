use axum::{extract::Query, Json};
use inspirer_foundation::service::Service;
use inspirer_foundation::Result;

use crate::app::{
    model::{content::Content, Paginate, Paginated},
    service::content::ContentService,
};

pub async fn get_content_list(
    Query(paginate): Query<Paginate>,
    service: Service<ContentService>,
) -> Result<Json<Paginated<Content>>> {
    service.get_content_list(paginate).await.map(Json)
}
