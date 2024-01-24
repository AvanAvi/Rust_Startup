fn main(){
    let course = ("rust", "programming","Language");

    if let ("rust",b,c)= course{
        println!("pattern matched with giuessed values : {} {}", b,c);
        
    }
    else {
        println!("pattern not matched");
    }
    }       

