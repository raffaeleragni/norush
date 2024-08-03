use axum_test::TestServer;
use norush::app;

#[tokio::test]
async fn test() {
    let app = TestServer::new(app::app()).unwrap();
    let index = app.get("/").await;
    println!("{}", index.text());
}
