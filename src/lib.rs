use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;
use std::net::TcpListener;

#[derive(Serialize)]
struct MessageResponse {
    message: String,
}

#[derive(Serialize)]
struct HealthCheckResponse {
    status: String,
    database: bool,
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

async fn check_database_connection() -> bool {
    // TODO: Logica para verificar se o banco esta saudável
    true
}
async fn get_message() -> impl Responder {
    HttpResponse::Ok().json(MessageResponse {
        message: "Welcome to Rust Server of Modu.lite App!".to_string(),
    })
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_message))
            .route("/health", web::get().to(health_check))
            .route("/get_message", web::get().to(get_message))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
