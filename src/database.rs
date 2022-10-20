use crate::schema;
use diesel::{prelude::*, Connection, SqliteConnection};
use rand::prelude::*;
use schema::words::dsl::*;

fn establish_connection() -> SqliteConnection {
    SqliteConnection::establish("words.db").expect("Failed to Create Connection")
}

pub fn get_words(length: i32) -> Vec<String> {
    let mut conn = establish_connection();

    words
        .filter(word_length.eq(length))
        .select(word)
        .load::<String>(&mut conn)
        .unwrap()
}

pub fn get_no_of_words() -> usize {
    let mut conn = establish_connection();
    words.count().execute(&mut conn).expect("Failed to Query")
}

pub fn get_random_word() -> String {
    let mut conn = establish_connection();
    let count = get_no_of_words();

    let mut rng = thread_rng();

    let num: i32 = rng.gen_range(0..count) as i32;

    let query_words: Vec<String> = words
        .filter(id.eq(num))
        .select(word)
        .load::<String>(&mut conn)
        .expect("Failed to Query");

    query_words.first().expect("Failed to query word!").clone()
}
