use axum::extract::Extension;
use inspirer_foundation::axum::app::{App, run_standard_cli_app};
use inspirer_foundation::Result;
use sea_orm::Database;

fn main() -> Result<()> {
    run_standard_cli_app("inspirer-blog", App::new(|config| async move {
        let router = blog::app::controller::http::routes();
        Ok(router.layer(Extension(
            Database::connect(config.get_string("database.connection")?.as_str())
                .await
                .unwrap(),
        )))
    }))
}
