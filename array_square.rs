fn main(){
    let mut avi = [1,2,3,4,5];
    println!("the original arrayb looks like {:?}",avi);
    square(&mut avi);
    println!("The array after squaring looks like {:?}", avi);
}

fn square(avi : &mut [i32;5]){
    for i in 0..5{
        avi[i] = avi[i] * avi[i];

    }
    println!("The array after squaring inside the function looks like {:?}", avi);
}