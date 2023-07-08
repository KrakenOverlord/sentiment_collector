use anyhow::Result;
use sqlx::{MySql, mysql::MySqlPoolOptions, Pool};

#[derive(Debug)]
pub struct SentimentEvent {
    pub id:         String,
    pub sentiment:  f32,
}

#[derive(Debug)]
pub struct Database {
    pool: Pool<MySql>,
}

impl Database {
    pub async fn new() -> Result<Self> {
        let url = std::env::var("DATABASE_URL")?;
        let pool = MySqlPoolOptions::new()
            .max_connections(10)
            .connect(&url).await?;

        Ok(Database { pool })
    }

    pub async fn record_event(&self, event: &SentimentEvent) -> Result<()> {
        let fields = "event_id, sentiment";
        let values = "?, ?";
        let query = format!("INSERT INTO events ({}) VALUES ({})", fields, values);
        sqlx::query(&query)
            .bind(&event.id)
            .bind(&event.sentiment)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
