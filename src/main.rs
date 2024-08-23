use modulite::repository::Repository;

use dotenv::dotenv;
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

    let listener = TcpListener::bind("127.0.0.1:8000")?;
    println!("Server is running on http://127.0.0.1:8000");

    run(listener, repository)?.await
}
