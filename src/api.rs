use crate::{
    database::{get_random_word, get_words},
    models,
};
use actix_web::{get, web, HttpResponse, Responder, Result};

#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("API \n/words\n{\nword_length\n}")
}

pub async fn word(length: web::Json<models::WordLengthQuery>) -> Result<impl Responder> {
    let words = get_words(length.0.length);

    Ok(web::Json(words))
}

pub async fn random_word() -> Result<impl Responder> {
    let word_result = get_random_word();
    Ok(web::Json(word_result))
}
