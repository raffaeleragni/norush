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
        .login_flow_with_mail(&db)
        .await
        .inject(db)
        .inject(mailer())
        .statics::<S>()
        .start()
        .await?;
    Ok(())
}
