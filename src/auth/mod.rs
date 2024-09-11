use crate::repository::Repository;
use actix_web::dev::ServiceRequest;
use actix_web::error::ErrorUnauthorized;
use actix_web::{web, Error, HttpResponse, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use apple_signin::AppleJwtClient;
use chrono::{Duration, Utc};
use jsonwebtoken::{
    decode, encode, errors::Error as JwtError, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct AppleLoginRequest {
    pub id_token: String,
}

#[derive(Serialize)]
pub struct AppleLoginResponse {
    pub token: String,
}

pub fn generate_jwt(username: &str) -> Result<String, JwtError> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let expiration = Utc::now()
        .checked_add_signed(Duration::days(1))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: username.to_owned(),
        exp: expiration as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
}

pub async fn login(
    repository: web::Data<Arc<Repository>>,
    login_data: web::Json<LoginRequest>,
) -> impl Responder {
    println!("Login Request");
    let user = repository.get_user_by_username(&login_data.username).await;
    match user {
        Ok(user) => {
            if repository
                .verify_password(&login_data.password, &user.password_hash)
                .await
            {
                match generate_jwt(&user.username) {
                    Ok(token) => {
                        println!("Token generated successfully: \n{token}");
                        HttpResponse::Ok().json(token)
                    }
                    Err(_) => HttpResponse::InternalServerError().finish(),
                }
            } else {
                println!("Invalid credentials");
                HttpResponse::Unauthorized().body("Invalid credentials")
            }
        }
        Err(_) => HttpResponse::Unauthorized().body("Invalid credentials"),
    }
}

pub async fn validate_jwt(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let token_data = decode::<Claims>(
        credentials.token(),
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    );

    match token_data {
        Ok(token) => {
            let user = token.claims.sub;
            println!("\nUser validated: {user}");
            Ok(req)
        }
        Err(_) => Err((ErrorUnauthorized("Invalid token"), req)),
    }
}

pub async fn apple_login(
    repository: web::Data<Arc<Repository>>,
    login_data: web::Json<AppleLoginRequest>,
) -> impl Responder {
    let client_id = env::var("APPLE_CLIENT_ID").expect("APPLE_CLIENT_ID must be set");

    let mut apple_client = AppleJwtClient::new(&[&client_id]);

    match apple_client.decode(&login_data.id_token).await {
        Ok(payload) => {
            if let Some(email) = payload.email {
                let user = repository.get_user_by_email(&email).await;

                match user {
                    Ok(user) => match generate_jwt(&user.username) {
                        Ok(token) => HttpResponse::Ok().json(AppleLoginResponse { token }),
                        Err(_) => HttpResponse::InternalServerError().finish(),
                    },
                    Err(_) => HttpResponse::Unauthorized().body("Usuário não encontrado"),
                }
            } else {
                HttpResponse::Unauthorized().body("Email não presente no ID Token da Apple")
            }
        }
        Err(_) => HttpResponse::Unauthorized().body("ID Token inválido"),
    }
}
