use actix_web::{web, App, HttpServer};
use config::connect_db;

mod config;
mod routes;
mod handlers;
mod database;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let pool = connect_db().await.expect("Failed to connect to database");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
