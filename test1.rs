fn main() {    
    let course = ("Rust", "beginner","course");
    if let ("Rust", "beginner","course") = course {
        println!("Wrote all values in pattern to be matched with the scrutinee expression");
    } else {
        println!("Value unmatched");
    }
}