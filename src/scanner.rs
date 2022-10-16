use std::collections::HashMap;
use std::io::BufRead;
use std::str::{from_utf8_unchecked, FromStr, SplitWhitespace};

use crate::database::get_words;

pub struct Scanner<R> {
    reader: R,
    buf_str: Vec<u8>,
    buf_iter: SplitWhitespace<'static>,
}

impl<R: BufRead> Scanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buf_str: vec![],
            buf_iter: "".split_whitespace(),
        }
    }
    pub fn cin<T: FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok().expect("Failed to Parse!");
            }
            self.buf_str.clear();
            self.reader
                .read_until(b'\n', &mut self.buf_str)
                .ok()
                .expect("Failed to Read!");
            self.buf_iter = unsafe {
                let slice = from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_whitespace())
            }
        }
    }
}

pub fn print_vec(words: &Vec<String>) {
    for word in words.iter() {
        println!("{}", word);
    }
}

trait Contains {
    fn contains(&self, chr: char) -> bool;
}

impl Contains for String {
    fn contains(&self, rhs: char) -> bool {
        for lhs in self.chars() {
            if lhs == rhs {
                return true;
            }
        }
        return false;
    }
}

pub fn contains_at(words: &Vec<String>, char_pos: &HashMap<char, (usize, i32)>) -> Vec<String> {
    let mut filtered_words: Vec<String> = Vec::with_capacity(words.len());

    for i in 0..words.len() {
        let mut to_be_inserted: bool = true;
        let current_word = words[i].clone();
        for ch in char_pos.keys() {
            let pos = char_pos.get(ch).unwrap();
            if pos.1 == -1 {
                if current_word.contains(*ch) {
                    to_be_inserted = false;
                }
            } else if pos.1 == 0 {
                if !(current_word.chars().nth(pos.0).unwrap() != *ch && current_word.contains(*ch))
                {
                    to_be_inserted = false;
                }
            } else if pos.1 == 1 {
                if current_word.chars().nth(pos.0).unwrap() != *ch {
                    to_be_inserted = false;
                }
            } else {
                panic!("Not allowed");
            }
        }

        if to_be_inserted {
            println!("{}", current_word);
            filtered_words.push(current_word);
        }
    }
    filtered_words
}

fn _main() {
    let stdin = std::io::stdin();
    let mut s = Scanner::new(stdin.lock());

    println!("Wordle Word Length: ");
    let n: i32 = s.cin();
    println!("Number of tries: ");
    let mut m: i32 = s.cin();

    let mut words: Vec<String> = get_words(n);

    println!("Try any of these words as first word: ");
    print_vec(&words);

    while m > 0 {
        println!("How many characters to be filtered");
        let mut x: i32 = s.cin();
        let mut char_pos: HashMap<char, (usize, i32)> = HashMap::new();

        while x > 0 {
            println!("Character & Position, and if correct pos then 1 else 0");
            let (chr, pos, resp) = (s.cin::<char>(), s.cin::<usize>() - 1, s.cin::<i32>());

            char_pos.insert(chr, (pos, resp));

            x -= 1;
        }

        words = contains_at(&words, &char_pos);
        print_vec(&words);

        m -= 1;
    }
}
