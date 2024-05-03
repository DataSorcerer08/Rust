use std::io;


fn main() {
  println!("Hellow world!");

  let mut input = String::new();
  io::stdin().read_line(&mut input).expect("fsailed to read ther line");

  println!("{:?}",input );

}