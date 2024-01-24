fn main(){

    let mut var =2;
    let mut found = false;

    while !found {
        var +=1;
        println!("{}",var);
        if var % 2 ==1 {
            found = true;
        }
            println!("found {}",var);
        
        println!("while running demo reached here ");
    }
}