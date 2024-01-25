fn fact(num: i32) -> i32 {

    let mut fact = 1;
  
    if num < 0 {
      println!("Factorial of negative number not possible");
      return 0; // return 0 for invalid input
    }
  
    for i in 1..=num {
      fact = fact * i;
    }
  
    return fact; // return factorial value
  }
  
  fn main() {
    
    let num = -1;
  
    let result = fact(num); // store result
  
    if result == 0 {
      println!("Invalid input");
    } else {
      println!("The factorial is {}", result); 
    }
  
  }