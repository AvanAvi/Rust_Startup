fn area_param(x:i32, y:i32)->(i32,i32){
    let area = x*y;
    let periemter = 2*(x+y);
    println!("the area of quad is :{}",area);
    println!("the parimeter of the quad is :{}",periemter);

    return (area,periemter);
}

fn main(){
    let x = 5;
    let y = 6;
    println!("the value of x is :{}",x);
    println!("the value of y is :{}",y);
    area_param(x,y);
}