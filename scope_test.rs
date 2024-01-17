fn main() {
    let x = 1000;
    let y = "Programming";
    println!("x: {}",x);
    println!("y: {}",y);
    {
        let x = 1100;
        println!("x: {}",x);
        println!("y: {}",y)
    }
   
   }