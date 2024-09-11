use dotenv::dotenv;
use modulite::repository::Repository;
use modulite::run;
use std::env;
use std::net::TcpListener;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set and valid");
    println!("DATABASE_URL: {}", db_url);
    let repository = Repository::new(&db_url)
        .await
        .expect("Failed to create repository: verify DATABASE_URL and SSL settings");
    let repository = Arc::new(repository);
    let port = env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let address = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&address)?;
    println!("Server is running on http://{}", address);
    run(listener, repository)?.await
}
