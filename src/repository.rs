use sqlx::{PgPool, Error, postgres::PgPoolOptions};
use bcrypt::{hash, DEFAULT_COST};

#[derive(Clone)]
pub struct Repository {
    pool: PgPool,
}

impl Repository {
    pub async fn new(database_url: &str) -> Result<Self, Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await;

        match pool {
            Ok(pool) => {
                println!("Database connection established successfully.");

                Self::create_user_table(&pool).await.expect("Failed to create user table");
                Ok(Repository { pool })
            },
            Err(e) => {
                eprintln!("Failed to connect to database: {:?}", e);
                Err(e)
            }
        }
    }
    pub async fn check_database_connection(&self) -> bool {
        match sqlx::query("SELECT 1").fetch_one(&self.pool).await {
            Ok(_) => {
                println!("Database connection is healthy.");
                true
            },
            Err(e) => {
                eprintln!("Database connection check failed: {:?}", e);
                false
            }
        }
    }

    pub async fn hash_password(&self, password: &str) -> Result<String, Error> {
        let hashed_password = hash(password, DEFAULT_COST).unwrap();
        Ok(hashed_password)
    }

    pub async fn create_user(&self, username: &str, email: &str, password_hash: &str) -> Result<(), Error> {
        let query = r#"
            INSERT INTO users (username, email, password_hash)
            VALUES ($1, $2, $3);
        "#;
        sqlx::query(query)
            .bind(username)
            .bind(email)
            .bind(password_hash)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
    pub async fn create_user_table(pool: &PgPool) -> Result<(), sqlx::Error> {
        let query = r#"
            CREATE TABLE IF NOT EXISTS users (
                id SERIAL PRIMARY KEY,
                username VARCHAR NOT NULL,
                email VARCHAR NOT NULL,
                password_hash VARCHAR NOT NULL
            );
        "#;
        sqlx::query(query).execute(pool).await?;
        Ok(())
    }
}
