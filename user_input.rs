use std::io;

fn main() {
  let mut input = String::new();

  println!("Enter some text:");

  io::stdin()
    .read_line(&mut input)
    .expect("bla bla bla ");

  println!("You entered: {}", input);  
}