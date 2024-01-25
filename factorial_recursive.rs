fn main(){
    let n = 5;
    let fact = factorial(n);
    println!("the factorial of {} is {}",n,fact);
}

fn factorial(n:i64)->i64{
    if n==0 {
        return 1;
}
    else{
        return n*factorial(n-1);
    }
}