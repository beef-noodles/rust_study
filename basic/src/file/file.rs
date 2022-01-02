use std::io;
// use std::io::ErrorKind;
// use std::io::Read;
use std::fs::File;
use std::result::Result;

pub fn read_file() -> Result<String, io::Error> {
  let mut s = String::new();
  File::open("text.txt")?.read_to_string(&mut s);
  Ok(s)
}