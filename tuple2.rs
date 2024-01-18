fn main(){
    let mut student =("adam",23,"male","japan");
    println!("original student :{:?}",student);
    student.0 = "zeus";
    println!("modified student :{:?}",student);
}