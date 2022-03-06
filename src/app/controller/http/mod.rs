use axum::{Router, routing::get};
use self::content::get_content_list;

pub mod content;

pub fn routes() -> Router {
    Router::new()
        .route("/content", get(get_content_list))
}