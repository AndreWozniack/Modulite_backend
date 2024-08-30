use crate::repository::{Repository, User};
use actix_web::{web, HttpResponse, Responder};
use std::sync::Arc;


pub async fn get_users(repository: web::Data<Arc<Repository>>) -> impl Responder {
    println!("get_users");

    let users = repository.get_users().await;
    match users {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => {
            eprintln!("Failed to get users: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn get_user_by_id(
    repository: web::Data<Arc<Repository>>,
    id: web::Path<i32>,
) -> impl Responder {
    println!("get_user_by_id");
    let user = repository.get_user_by_id(id.into_inner()).await;
    match user {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => {
            eprintln!("Failed to get user by id: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn add_user(
    repository: web::Data<Arc<Repository>>,
    user: web::Json<User>,
) -> impl Responder {
    println!("add_user");

    let user = user.into_inner();
    let password_hash = repository.hash_password(&user.password_hash).await.unwrap();

    let result = repository
        .create_user(&user.username, &user.email, &password_hash)
        .await;

    match result {
        Ok(_) => HttpResponse::Created().finish(),
        Err(e) => {
            eprintln!("Failed to add user: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn update_user(
    repository: web::Data<Arc<Repository>>,
    id: web::Path<i32>,
    user: web::Json<User>,
) -> impl Responder {
    println!("update_user");
    let user = user.into_inner();
    let password_hash = repository.hash_password(&user.password_hash).await.unwrap(); // Gerando o hash da senha

    let result = repository
        .update_user(
            id.into_inner(),
            Some(&user.username),
            Some(&user.email),
            Some(&password_hash),
        )
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            eprintln!("Failed to update user: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn delete_user(
    repository: web::Data<Arc<Repository>>,
    id: web::Path<i32>,
) -> impl Responder {
    println!("delete_user");
    let result = repository.delete_user(id.into_inner()).await;
    match result {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => {
            eprintln!("Failed to delete user: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
