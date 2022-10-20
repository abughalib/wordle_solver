use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};

pub mod api;
pub mod database;
pub mod models;
pub mod scanner;
pub mod schema;

#[macro_use]
extern crate diesel;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_header()
                    .allowed_methods(vec!["GET", "POST"])
                    .max_age(3600),
            )
            .wrap(Logger::default())
            .service(api::index)
            .route("words", web::post().to(api::word))
            .route("random_word", web::post().to(api::random_word))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
