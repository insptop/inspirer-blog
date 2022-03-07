use axum::{Router, routing::get};
use self::content::*;

pub mod content;

pub fn routes() -> Router {
    Router::new()
        .route("/content", get(get_content_list))
        .route("/content/:index", get(get_content))
}