use std::net::TcpListener;
use Modulite::run;


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("0.0.0.0:8080")
        .expect("Failed to bind port 8080");
    run(listener)?.await
}


