use crate::repository::Repository;
use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;
use std::net::TcpListener;
use std::sync::Arc;

pub mod repository;

#[derive(Serialize)]
struct MessageResponse {
    message: String,
}

#[derive(Serialize)]
struct HealthCheckResponse {
    status: String,
    database: bool,
}

async fn health_check(repository: web::Data<Arc<Repository>>) -> impl Responder {
    let database_status = repository.check_database_connection().await;
    let response = HealthCheckResponse {
        status: "Healthy".to_string(),
        database: database_status,
    };

    HttpResponse::Ok().json(response)
}

async fn get_message() -> impl Responder {
    HttpResponse::Ok().json(MessageResponse {
        message: "Welcome to Rust Server of Modu.lite App!".to_string(),
    })
}

async fn test_get_message() -> impl Responder {
    HttpResponse::Ok().json(MessageResponse {
        message: "Testing".to_string(),
    })
}

pub fn run(listener: TcpListener, repository: Arc<Repository>) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(repository.clone()))
            .route("/", web::get().to(get_message))
            .route("/health", web::get().to(health_check))
            .route("/get_message", web::get().to(get_message))
            .route("/test", web::get().to(test_get_message))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
