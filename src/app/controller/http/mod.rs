use self::content::*;
use axum::{routing::get, Router};

pub mod content;

pub fn routes() -> Router {
    Router::new().nest(
        "/api",
        Router::new()
            .route("/content", get(get_content_list))
            .route("/content/:index", get(get_content)),
    )
}
