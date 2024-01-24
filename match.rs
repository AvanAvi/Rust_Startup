fn main(){
    let course = "rust";

    let found_course = match course{
        "rust" => "rust",
        "java" => "JAVA",
        _=> " nada"
    };
println!("COURSE NAME : {} ",found_course);

}