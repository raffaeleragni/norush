use anyhow::Result;
use axum_test::TestServer;
use norush::app;

#[tokio::test]
async fn test() -> Result<()> {
    let app = TestServer::new(app().await?)?;
    let index = app.get("/").await;
    println!("{}", index.text());
    Ok(())
}
