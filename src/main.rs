use anyhow::Result;
use norush::app;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    println!("xdg-open http://127.0.0.1:3000/");
    axum::serve(TcpListener::bind("127.0.0.1:3000").await?, app().await?).await?;
    Ok(())
}
