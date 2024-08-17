use sqlx::{postegres::PgPoolOptions, Pool, Postegres};


pub async fn start_connection() -> Poll<Postegres>{
    let postgres_enviroment = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&postgres_enviroment)
        .await
        .expect("Falied to connect to Postegres");

}