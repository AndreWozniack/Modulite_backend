use crate::repository::Repository;
use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::net::TcpListener;
use std::sync::Arc;

pub mod repository;
pub mod users;

#[derive(Serialize, Deserialize)]
pub struct MessageResponse {
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct HealthCheckResponse {
    pub status: String,
    pub database: bool,
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
async fn test_db_connection(repository: web::Data<Arc<Repository>>) -> impl Responder {
    if repository.check_database_connection().await {
        HttpResponse::Ok().body("Database is healthy!")
    } else {
        HttpResponse::InternalServerError().body("Failed to connect to database!")
    }
}

pub fn run(listener: TcpListener, repository: Arc<Repository>) -> Result<Server, std::io::Error> {
    env_logger::init();
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(repository.clone()))
            .route("/", web::get().to(get_message))
            .route("/health", web::get().to(health_check))
            .route("/test_db", web::get().to(test_db_connection))
            // TODO: Add routes to handle CRUD operations for user and widget configurations
            .route("/users", web::get().to(users::get_users))
            .route("/users/{id}", web::get().to(users::get_user_by_id))
            .route("/users", web::post().to(users::add_user))
            .route("/users/{id}", web::delete().to(users::delete_user))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
