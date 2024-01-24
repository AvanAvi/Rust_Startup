fn main(){


calculate(3,'+',2);
calculate(3,'(',2);
calculate(3,'*',2);
calculate(3,'/',2);

}

fn calculate(a: i32, operator : char, b: i32){
    match operator {
        '+' => println!("{} + {} = {}",a,b,a+b),
        '-' => println!("{} - {} = {}",a,b,a-b),
        '*' => println!("{} * {} = {}",a,b,a*b),
        '/' => {
            if b ==0 {
                println!("division by 0 is undefined ");

            }
            else {
                println!("{} / {} = {}",a,b,a/b);
            }
        },
        _=>println!("Unknown operation character"),
    }
}