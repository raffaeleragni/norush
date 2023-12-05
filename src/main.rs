mod ui;
mod db;
mod statics;

use anyhow::Result;
use axum::{Extension, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    let tcp = TcpListener::bind("127.0.0.1:3000").await?;
    println!("xdg-open http://127.0.0.1:3000/");
    axum::serve(tcp, app().await?).await?;
    Ok(())
}

async fn app() -> Result<Router> {
    let mut app = Router::new();
    app = ui::init(app);
    app = statics::init(app);
    Ok(app.layer(Extension(db::init().await?)))
}
