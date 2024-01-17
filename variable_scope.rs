fn main() {
    let inner = 5;
    let outer = 12;
    {
        let inner = 10;
        println!("Inner: {}", inner);
        println!("Outer: {}", outer);
    }
    println!("Inner: {}", inner);
    println!("Outer: {}", outer);
}