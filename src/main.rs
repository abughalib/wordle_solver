use actix_web::{web, App, HttpServer};

pub mod api;
pub mod database;
pub mod models;
pub mod scanner;
pub mod schema;

#[macro_use]
extern crate diesel;

// fn print_vec(words: &Vec<String>) {
//     for word in words.iter() {
//         println!("{}", word);
//     }
// }

// trait Contains {
//     fn contains(&self, chr: char) -> bool;
// }

// impl Contains for String {
//     fn contains(&self, rhs: char) -> bool {
//         for lhs in self.chars() {
//             if lhs == rhs {
//                 return true;
//             }
//         }
//         return false;
//     }
// }

// fn contains_at(words: &Vec<String>, char_pos: &HashMap<char, (usize, i32)>) -> Vec<String> {
//     let mut filtered_words: Vec<String> = Vec::with_capacity(words.len());

//     for i in 0..words.len() {
//         let mut to_be_inserted: bool = true;
//         let current_word = words[i].clone();
//         for ch in char_pos.keys() {
//             let pos = char_pos.get(ch).unwrap();
//             if pos.1 == -1 {
//                 if current_word.contains(*ch) {
//                     to_be_inserted = false;
//                 }
//             } else if pos.1 == 0 {
//                 if !(current_word.chars().nth(pos.0).unwrap() != *ch && current_word.contains(*ch))
//                 {
//                     to_be_inserted = false;
//                 }
//             } else if pos.1 == 1 {
//                 if current_word.chars().nth(pos.0).unwrap() != *ch {
//                     to_be_inserted = false;
//                 }
//             } else {
//                 panic!("Not allowed");
//             }
//         }

//         if to_be_inserted {
//             println!("{}", current_word);
//             filtered_words.push(current_word);
//         }
//     }
//     filtered_words
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let stdin = std::io::stdin();
    // let mut s = scanner::Scanner::new(stdin.lock());

    // println!("Wordle Word Length: ");
    // let n: i32 = s.cin();
    // println!("Number of tries: ");
    // let mut m: i32 = s.cin();

    // let mut words: Vec<String> = get_words(n);

    // println!("Try any of these words as first word: ");
    // print_vec(&words);

    // while m > 0{

    //     println!("How many characters to be filtered");
    //     let mut x: i32 = s.cin();
    //     let mut char_pos: HashMap<char, (usize, i32)> = HashMap::new();

    //     while x > 0{

    //         println!("Character & Position, and if correct pos then 1 else 0");
    //         let (chr, pos, resp) = (s.cin::<char>(), s.cin::<usize>()-1, s.cin::<i32>());

    //         char_pos.insert(chr, (pos, resp));

    //         x-=1;
    //     }

    //     words = contains_at(&words, &char_pos);
    //     print_vec(&words);

    //     m -=1;
    // }

    HttpServer::new(|| 
        App::new().service(api::index)
        .route("word", web::post().to(api::word))
    )
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
