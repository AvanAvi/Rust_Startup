fn triangle(row: i32) {

    for i in 1..=row {
      for _ in 0..i {
        print!("&"); 
      }
  
      println!(""); // newline after each row
    }
  
  }
  
  fn main() {
  
    let row = 5;
    triangle(row);
  
  }