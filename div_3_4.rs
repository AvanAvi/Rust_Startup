fn main(){
    let number = 12;
    println!("the number entered for divisiblity check is {}",number);
    check(number);
}

fn check(number:i32){
    if number % 3 ==0 && number % 4 ==0 {
        println!("0");
    } 
    else if number % 3 ==0{
        println!("1");

    }
    else if number % 4 ==0{
        println!("2");
    }
    else {
        println!("-1");
    }
}