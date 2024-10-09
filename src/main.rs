mod app;

use velvet_web::prelude::*;

#[tokio::main]
async fn main() -> AppResult<()> {
    #[derive(RustEmbed)]
    #[folder = "static"]
    struct S;

    dotenvy::dotenv().ok();
    let db = sqlite().await;
    sqlx::migrate!().run(&db).await?;

    App::new()
        .router(app::app())
        .login_flow(&db)
        .await
        .inject(db)
        .statics::<S>()
        .start()
        .await?;
    Ok(())
}
