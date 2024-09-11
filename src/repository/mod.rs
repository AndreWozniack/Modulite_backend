use bcrypt::{hash, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, Error, FromRow, PgPool};

#[derive(Clone)]
pub struct Repository {
    pub pool: PgPool,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

impl Repository {
    pub(crate) async fn get_user_by_username(&self, username: &str) -> Result<User, Error> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
            .bind(username)
            .fetch_one(&self.pool)
            .await?;
        Ok(user)
    }

    pub(crate) async fn get_user_by_email(&self, email: &str) -> Result<User, Error> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_one(&self.pool)
            .await?;
        Ok(user)
    }
    pub(crate) async fn verify_password(&self, password: &str, password_hash: &str) -> bool {
        match bcrypt::verify(password, password_hash) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
    /// Create a new instance of the repository.
    pub async fn new(database_url: &str) -> Result<Self, Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await;

        match pool {
            Ok(pool) => {
                println!("Database connection established successfully.");

                Self::create_user_table(&pool)
                    .await
                    .expect("Failed to create user table");
                Ok(Repository { pool })
            }
            Err(e) => {
                eprintln!("Failed to connect to database: {:?}", e);
                Err(e)
            }
        }
    }
    pub(crate) async fn check_database_connection(&self) -> bool {
        match sqlx::query("SELECT 1").fetch_one(&self.pool).await {
            Ok(_) => {
                println!("Database connection is healthy.");
                true
            }
            Err(e) => {
                eprintln!("Database connection check failed: {:?}", e);
                false
            }
        }
    }

    pub(crate) async fn hash_password(&self, password: &str) -> Result<String, Error> {
        let hashed_password = hash(password, DEFAULT_COST).unwrap();
        Ok(hashed_password)
    }

    async fn create_user_table(pool: &PgPool) -> Result<(), sqlx::Error> {
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

    /// Get all users from the database.
    pub(crate) async fn get_users(&self) -> Result<Vec<User>, Error> {
        let users = sqlx::query_as::<_, User>("SELECT * FROM users")
            .fetch_all(&self.pool)
            .await?;
        Ok(users)
    }

    /// Get a user by ID from the database.
    pub(crate) async fn get_user_by_id(&self, user_id: i32) -> Result<User, Error> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(user_id)
            .fetch_one(&self.pool)
            .await?;
        Ok(user)
    }

    /// Create a new user in the database.
    pub(crate) async fn create_user(
        &self,
        username: &str,
        email: &str,
        password_hash: &str,
    ) -> Result<(), Error> {
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
        print!("User created successfully.");
        Ok(())
    }

    /// Update a user in the database.
    pub(crate) async fn update_user(
        &self,
        user_id: i32,
        username: Option<&str>,
        email: Option<&str>,
        password: Option<&str>,
    ) -> Result<(), Error> {
        let mut query = String::from("UPDATE users SET ");
        let mut has_set = false;

        if let Some(username) = username {
            query.push_str(&format!("username = '{}'", username));
            has_set = true;
        }

        if let Some(email) = email {
            if has_set {
                query.push_str(", ");
            }
            query.push_str(&format!("email = '{}'", email));
            has_set = true;
        }

        if let Some(password) = password {
            let password_hash = self.hash_password(password).await?;
            if has_set {
                query.push_str(", ");
            }
            query.push_str(&format!("password_hash = '{}'", password_hash));
        }

        query.push_str(&format!(" WHERE id = {};", user_id));

        sqlx::query(&query).execute(&self.pool).await?;
        Ok(())
    }

    /// Delete a user from the database.
    pub(crate) async fn delete_user(&self, user_id: i32) -> Result<(), Error> {
        let query = "DELETE FROM users WHERE id = $1";
        sqlx::query(query).bind(user_id).execute(&self.pool).await?;
        Ok(())
    }
}
