use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder,};
use paperclip::actix:: {OpenApiExt, Apiv2Schema, api_v2_operation};
use serde::{Serialize, Deserialize};


async fn health_checker(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error>{
    HttpServer::new( || {
        App::new()
            .route("/health_check", web::get().to(health_checker))
    })
        .bind("127.0.0.1:8000")
        ?.run()
        .await
}
