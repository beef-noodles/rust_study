
extern crate diesel_demo;
extern crate diesel;

use self::diesel_demo::*;
use std::io::{stdin};


fn main() {
  let connection = establish_connection();
  println!("What would you like your title to be?");
  let mut title = String::new();
  stdin().read_line(&mut title).unwrap();
  let title = &title[..title.len() - 1];
  println!("\nOk! Let's write {} (Press {} when finished)\n", title, EOF);
  println!("entry post body");
  let mut body = String::new();
  stdin().read_line(&mut body).unwrap();
  let body = &body[..body.len() - 1];
  let post = create_post(&connection, title, &body);
  println!("\n Saved draft {} with id {}", title, post.id)
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";
#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";