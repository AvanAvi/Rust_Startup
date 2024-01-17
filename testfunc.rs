fn main(){
    println!("{}",1);
    println!("{}",22);
    println!("{}",333);
    println!("{}",4444);
    println!("{}",55555);
    test()
}

fn test() {
    println!("{}", 1);
    println!("{}{}", 2, 2);
    println!("{}{}{}", 3, 3, 3);
    println!("{}{}{}{}", 4, 4, 4, 4);
    println!("{}{}{}{}{}", 5, 5, 5, 5, 5);
}