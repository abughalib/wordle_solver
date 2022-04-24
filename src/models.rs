use super::diesel::Queryable;

#[derive(Queryable)]
pub struct QueryWords{
  pub id: i32,
  pub word: String,
  pub word_length: i32
}