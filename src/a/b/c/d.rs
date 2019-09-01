
pub fn print_ddd () {
  println!("i'a ddd");
}

pub fn find(haystack: &str, needle: char) -> Option<usize> {
  for (offset, c ) in haystack.char_indices() {
    if c == needle {
      return Some(offset);
    }
  }
  None
}