fn main(){
    let fruit_name =("apple","kiwi","banana");

    if let ("pineapple",c,d) = fruit_name {
        println!("course is : {} {} ",c,d);
    }
    else{
        println!("pattern not matched");
    }
                                            
    
}