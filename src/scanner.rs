use std::io::BufRead;
use std::str::{SplitWhitespace, from_utf8_unchecked, FromStr};

pub struct Scanner<R> {
  reader: R,
  buf_str: Vec<u8>,
  buf_iter: SplitWhitespace<'static>
}

impl<R: BufRead> Scanner<R>{
  pub fn new(reader: R)-> Self{
    Self {
      reader,
      buf_str: vec![],
      buf_iter: "".split_whitespace()
    }
  }
  pub fn cin<T: FromStr>(&mut self) -> T{
    loop{
      if let Some(token) = self.buf_iter.next(){
        return token.parse().ok().expect("Failed to Parse!");
      }
      self.buf_str.clear();
      self.reader.read_until(b'\n',&mut self.buf_str)
      .ok().expect("Failed to Read!");
      self.buf_iter = unsafe {
        let slice = from_utf8_unchecked(&self.buf_str);
        std::mem::transmute(slice.split_whitespace())
      }
    }
  }
}