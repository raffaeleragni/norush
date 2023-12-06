mod db;
mod statics;
mod ui;

use anyhow::Result;
use axum::{Extension, Router};

pub async fn app() -> Result<Router> {
    let mut app = Router::new();
    app = ui::init(app);
    app = statics::init(app);
    Ok(app.layer(Extension(db::init().await?)))
}
