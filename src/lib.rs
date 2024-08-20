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
    // Simulação de uma verificação de banco de dados
    let db_healthy = check_database_connection().await;

    let overall_status = if db_healthy {
        "Healthy".to_string()
    } else {
        "Unhealthy".to_string()
    };

    HttpResponse::Ok().json(HealthCheckResponse {
        status: overall_status,
        database: db_healthy,
    })
}

async fn check_database_connection() -> bool {
    // TODO: Logica para verificar se o banco esta saudável
    true
}
async fn get_message() -> impl Responder {
    HttpResponse::Ok().json(MessageResponse {
        message: "Bem vindo ao servidor do Modu.lite em Rust!".to_string(),
    })
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/get_message", web::get().to(get_message))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
