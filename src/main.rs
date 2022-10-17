use actix_web::{web, App, HttpServer};

pub mod api;
pub mod database;
pub mod models;
pub mod scanner;
pub mod schema;

#[macro_use]
extern crate diesel;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(api::index)
            .route("word", web::post().to(api::word))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
