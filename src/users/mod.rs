use crate::repository::{Repository, User};
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub async fn get_users(repository: web::Data<Arc<Repository>>) -> impl Responder {
    println!("Get Users Request");

    let users = repository.get_users().await;
    match users {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => {
            eprintln!("Failed to get users: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to get users")
        }
    }
}

pub async fn get_user_by_id(
    repository: web::Data<Arc<Repository>>,
    id: web::Path<i32>,
) -> impl Responder {
    println!("Get User Request");
    let user = repository.get_user_by_id(id.into_inner()).await;
    match user {
        Ok(user) => {
            println!("User found: {:?}", user);
            HttpResponse::Ok().json(user)
        }

        Err(e) => {
            eprintln!("Failed to get user by id: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to get user by id")
        }
    }
}

pub async fn add_user(
    repository: web::Data<Arc<Repository>>,
    user: web::Json<CreateUserRequest>,
) -> impl Responder {
    println!("Add User Request: {:?}", user);

    let user = user.into_inner();
    let password_hash = repository.hash_password(&user.password).await.unwrap();

    let result = repository
        .create_user(&user.username, &user.email, &password_hash)
        .await;

    match result {
        Ok(_) => {
            println!("User created successfully!");
            HttpResponse::Created().body("User created successfully!")
        }
        Err(e) => {
            eprintln!("Failed to add user: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to add user")
        }
    }
}

pub async fn update_user(
    repository: web::Data<Arc<Repository>>,
    id: web::Path<i32>,
    user: web::Json<User>,
) -> impl Responder {
    println!("Update User Request: {:?}", user);
    let user = user.into_inner();
    let password_hash = repository.hash_password(&user.password_hash).await.unwrap();

    let result = repository
        .update_user(
            id.into_inner(),
            Some(&user.username),
            Some(&user.email),
            Some(&password_hash),
        )
        .await;

    match result {
        Ok(_) => {
            println!("User updated successfully!");
            HttpResponse::Ok().body("User updated successfully!")
        }
        Err(e) => {
            eprintln!("Failed to update user: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to update user")
        }
    }
}

pub async fn delete_user(
    repository: web::Data<Arc<Repository>>,
    id: web::Path<i32>,
) -> impl Responder {
    println!("Delete User Request, id: {:?}", id);
    let result = repository.delete_user(id.into_inner()).await;
    match result {
        Ok(_) => {
            println!("User deleted successfully!");
            HttpResponse::NoContent().body("User deleted successfully!")
        }
        Err(e) => {
            eprintln!("Failed to delete user: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to delete user")
        }
    }
}
