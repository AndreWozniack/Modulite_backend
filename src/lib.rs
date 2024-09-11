use crate::repository::Repository;
use actix_web::dev::Server;
use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use actix_web_httpauth::middleware::HttpAuthentication;
use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::net::TcpListener;
use std::sync::Arc;
pub mod auth;
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
        info!("Database is healthy!");
        HttpResponse::Ok().body("Database is healthy!")
    } else {
        warn!("Failed to connect to database!");
        HttpResponse::InternalServerError().body("Failed to connect to database!")
    }
}

fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .wrap(HttpAuthentication::bearer(auth::validate_jwt))
            .route("", web::get().to(users::get_users))
            .route("/{id}", web::get().to(users::get_user_by_id))
            .route("", web::post().to(users::add_user))
            .route("/{id}", web::delete().to(users::delete_user))
            .route("/{id}", web::put().to(users::update_user)),
    );
}

fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/auth").route("/login", web::post().to(auth::login)));
}

pub fn run(listener: TcpListener, repository: Arc<Repository>) -> Result<Server, std::io::Error> {
    env_logger::init();
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(repository.clone()))
            // Rotas de autenticação
            .configure(auth_routes)
            // Rotas de teste
            .route("/", web::get().to(get_message))
            .route("/health", web::get().to(health_check))
            .route("/test_db", web::get().to(test_db_connection))
            // Rotas de usuários
            .configure(user_routes)

        // TODO: adicionar rotas para salvar e obter preferências do usuário
        // TODO: adicionar rotas para salvar e obter widgets
    })
    .listen(listener)?
    .run();

    Ok(server)
}
