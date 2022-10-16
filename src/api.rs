use actix_web::{get, Responder, HttpResponse, web, Result};
use crate::{models, database::get_words};

#[get("/")]
pub async fn index() -> impl Responder {
  HttpResponse::Ok().body("API \n/words\n{\nword_length\n}")
}

pub async fn word(length: web::Json<models::WordLengthQuery>) -> Result<impl Responder> {
  let words = get_words(length.0.length);

  Ok(web::Json(words))
}