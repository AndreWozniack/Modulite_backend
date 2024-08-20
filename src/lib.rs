use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::dev::Server;
use std::net::TcpListener;
use serde::Serialize;

async fn health_check() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/plain")
        .body("Health check passed!")
}

#[derive(Serialize)]
struct MessageResponse {
    message: String,
}

async fn get_message() -> impl Responder {
    HttpResponse::Ok().json(MessageResponse {
        message: "Bem vindo ao servidor do Modu.lite em Rust!".to_string()
    })
}


pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> { let server = HttpServer::new(|| {
    App::new()
        .route("/health_check", web::get().to(health_check))
        .route("/get_message", web::get().to(get_message))

})
    .listen(listener)?
    .run();
    Ok(server)
}

