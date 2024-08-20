use std::net::TcpListener;
use modulite::run;


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("0.0.0.0:80")
        .expect("Failed to bind port 80");
    run(listener)?.await
}


