use diesel::{SqliteConnection, Connection, prelude::*};
use crate::schema;

pub fn get_words(length: i32) -> Vec<String> {
  use schema::words::dsl::*;
  let mut conn = SqliteConnection::establish("words.db").expect("Failed create connections");

  words
    .filter(word_length.eq(length))
    .select(word)
    .load::<String>(&mut conn)
    .unwrap()
}