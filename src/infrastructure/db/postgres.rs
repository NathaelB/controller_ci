use sqlx::PgPool;
use std::sync::Arc;

pub struct Postgres {
    pub pool: Arc<PgPool>,
}

impl Postgres {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPool::connect(database_url).await?;

        Ok(Self {
            pool: Arc::new(pool),
        })
    }

    pub fn get_pool(&self) -> Arc<PgPool> {
        Arc::clone(&self.pool)
    }
}
