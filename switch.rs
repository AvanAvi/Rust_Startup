fn main() {
    let mut a = 2;
    let mut b = 3;
    a += a;
    b -= b;
    a *= 1;
    b *= 3;
    a -= 1;
    println!("a: {}", a);
    println!("b: {}", b); 
  }