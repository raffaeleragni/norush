use anyhow::Result;
use sqlx::{migrate::MigrateDatabase, Pool, Sqlite, SqlitePool};

static DB_URL: &str = "sqlite://norush.db";

pub async fn init() -> Result<Pool<Sqlite>> {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        Sqlite::create_database(DB_URL).await?
    }
    let db = SqlitePool::connect(DB_URL).await?;
    sqlx::migrate!().run(&db).await?;
    Ok(db)
}
