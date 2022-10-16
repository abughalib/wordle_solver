use actix_web::{get, post, Responder, HttpResponse, web};
use crate::models;

#[get("/")]
pub async fn index() -> impl Responder {
  HttpResponse::Ok().body("API \n/words\n{\nword_length\n}")
}

pub async fn word(length: web::Json<models::QueryWords>) -> impl Responder {
  HttpResponse::Ok().body("body")
}