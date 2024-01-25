fn main(){
    let n=7;
    let fib = fibonacci(n);
    println!("the fibonacci of {} is {}",n,fib);   
}

fn fibonacci(n:i32)->i32{
    if n==0{
        return 0;
    }
    else if n==1{
        return 1;
    }
    else{
        return fibonacci(n-1)+fibonacci(n-2);
    }
}