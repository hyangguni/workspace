
#![allow(unused)]

struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool,
}

fn main() {
  let mut user1 = User {
    email : String::from("hyangguni@hanmail.net"),
    username : String::from("최향태"),
    active: true,
    sign_in_count:1,
  };

  user1.email = String::from("choi.hyangtae@yhsbearing.com");

  println!("eamil is {}",user1.email);


}

