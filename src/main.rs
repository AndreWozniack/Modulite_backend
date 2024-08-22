use modulite::repository::Repository;

// use actix_web::dev::Server;
use dotenv::dotenv;
use modulite::run;
use std::env;
use std::net::TcpListener;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let repository = Repository::new(&database_url)
        .await
        .expect("Failed to create repository");
    let repository = Arc::new(repository);

    let listener = TcpListener::bind("127.0.0.1:8000")?;
    run(listener, repository)?.await
}
