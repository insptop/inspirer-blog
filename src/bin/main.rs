use axum::extract::Extension;
use inspirer_foundation::axum::app::App;
use sea_orm::Database;

#[tokio::main]
async fn main() {
    let router = blog::app::controller::http::routes();
    
    App {
        router: router.layer(Extension(Database::connect("mysql://root:root@localhost/inspirer_blog").await.unwrap()))
    }.run().await;
}