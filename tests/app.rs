use velvet_web::prelude::*;
use norush::app;

#[tokio::test]
async fn test() {
    let app = app::app().await.as_test_server().await.login_as("1").await;
    let index = app.get("/").await;
    index.assert_status_ok();
}
