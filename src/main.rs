use modulite::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("0.0.0.0:8080").expect("Failed to bind port 80");
    run(listener)?.await
}
