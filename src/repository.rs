use sqlx::Error;
use sqlx::PgPool;

#[derive(Clone)]
pub struct Repository {
    pool: PgPool,
}

impl Repository {
    pub async fn new(database_url: &str) -> Result<Self, Error> {
        let pool = PgPool::connect(database_url).await?;
        Ok(Repository { pool })
    }

    pub async fn check_database_connection(&self) -> bool {
        sqlx::query("SELECT 1").fetch_one(&self.pool).await.is_ok()
    }

    // Adicione outros métodos para interagir com o banco de dados aqui.
}
