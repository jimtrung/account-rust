use actix_web::{delete, get, web, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;

use crate::database::users::{delete_user_by_id, get_user_by_id, get_users};

#[get("/info")]
pub async fn list_users(pool: web::Data<PgPool>) -> impl Responder {
    match get_users(&pool).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().body("Failed to fetch users")
    }
}

#[get("/info/{user_id}")]
pub async fn get_user(user_id: web::Path<String>, pool: web::Data<PgPool>) -> impl Responder {
    let user_id = match Uuid::parse_str(&user_id) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid UUID format")
    };

    match get_user_by_id(&pool, (user_id) as Uuid).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().body("Failed to get user")
    }
}

#[delete("/delete/{user_id}")]
pub async fn delete_user(user_id: web::Path<String>, pool: web::Data<PgPool>) -> impl Responder {
    let user_id = match Uuid::parse_str(&user_id) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid UUID format")
    };

    match delete_user_by_id(&pool, (user_id) as Uuid).await {
        Ok(msg) => HttpResponse::Ok().json(msg),
        Err(_) => HttpResponse::InternalServerError().body("Failed to delete user")
    }
}
