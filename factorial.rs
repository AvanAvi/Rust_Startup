fn fact(number:i32)->i32{
    let mut fact =1;

    if number < 0 {
        println!("factorial of negative number is not possible");
       
    }
    else if number ==0 {
        println!("1")
    }
    else {

    
            for i in 1..number {
        fact = fact*i;
    }}
    fact
   
}
fn main(){
    let number = -1;
    println!("the factorial of the given number is {}",fact(number));
}