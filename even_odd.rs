fn main(){

    test(5);
}

fn test(number:i32){

    if number % 2 ==0{
        println!("given number {} is even",number);
    }

    else {
        println!("given number {} is odd",number);
    }
}