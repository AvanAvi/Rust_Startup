fn main() {
    let mut a = false;
    let mut b = true;
    a = a && b || ( ! a);
    b = !b;
    println!("a:{}", a);
    println!("b:{}", b); 
  }