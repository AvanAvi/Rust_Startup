fn main(){
    let x = 5;
    let mut y = 12;

    let a = &x;
    println!("a is {}",a);
    println!("x is {}",x);

    let b = &mut y;
    println!("b is {}",b);

    *b = 521;
    println!("b is {}",b);
    println!("y is {}",y);
    
}