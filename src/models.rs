use serde_derive::{Serialize, Deserialize};

use super::diesel::Queryable;

#[derive(Queryable)]
pub struct QueryWords{
  pub id: i32,
  pub word: String,
  pub word_length: i32
}

#[derive(Deserialize)]
pub struct WordLengthQuery {
  pub length: i32,
}

#[derive(Serialize)]
pub struct Words{
  pub words: Vec<String>
}