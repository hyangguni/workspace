#![allow(unused)]
fn main() {
  let s1 = String::from("he  llo");

  let len = first_word(&s1);

  println!("test {}", len);
}
fn first_word(s: &String) -> usize {
  let bytes = s.as_bytes();
  for (i, &item) in bytes.iter().enumerate() {
      if item == b'e' {
          return i;
      }
  }

  s.len()
}

