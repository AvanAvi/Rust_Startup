fn square(n:&mut i32){
    *n = *n * *n;
    println!("the value of n inside the fucn is :{}",n);

}

fn main(){
   let mut n = 5;
    println!("the value of n before the func is called is :{}",n);
    println!("function invoked");
    square(&mut n);
    println!("the value of n after the function is being called is: {}",n);
    println!("{}",n);
}