fn main(){

    let person = ("Adam",23,"male","british");
    println!("Person name and nationality is : {} {}",person.0, person.3);
    let (x,y,z,k) = person;
    println!("x: {}",x);
    println!("y: {}",y);
    println!("z: {}",z);
    println!("k: {}",k);
    

}