mod app;

use velvet_web::prelude::*;

#[tokio::main]
async fn main() -> AppResult<()> {
    #[derive(RustEmbed)]
    #[folder = "static"]
    struct S;

    JWT::Secret.setup().await?;

    dotenvy::dotenv().ok();
    let db = sqlite().await;
    login_setup(&db).await?;
    sqlx::migrate!().run(&db).await?;

    App::new()
        .router(app::app())
        .inject(db)
        .statics::<S>()
        .start()
        .await?;
    Ok(())
}
