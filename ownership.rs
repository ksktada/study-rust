fn main() {
  let s1 = String::from("hello");
  let s2 = s1.clone();

  println!("s1 = {}, s2 = {}", s1, s2);
}
// This code does not compile
// s1 doesn't have ownership.
// fn main() {
//  let s1 = String::from("hello");
//  let s2 = s1;
//
//  println!("{}, world!", s1);
//}
