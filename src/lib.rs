use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder,};
use actix_web::dev::Server;
use std::net::TcpListener;
use paperclip::actix:: {OpenApiExt, Apiv2Schema, api_v2_operation};
use serde::{Serialize, Deserialize};


#[derive(serde::Deserialize)] struct FormData {
    email: String,
    name: String
}

async fn health_check() -> HttpResponse { 
    HttpResponse::Ok().finish() 
}

async fn subscribe(_form: web::Form<FormData>) -> HttpResponse { 
    HttpResponse::Ok().finish() 
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> { let server = HttpServer::new(|| {
    App::new()
        .route("/health_check", web::get().to(health_check))
        .route("/subscriptions", web::post().to(subscribe))
})
    .listen(listener)?
    .run();
    Ok(server) 
}

