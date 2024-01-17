fn main() {
    let arr = [1, 2, 3, 4];
  
    let slice1 = &arr[..2]; 
    let slice2 = arr.as_slice();
  
    println!("Slice: {:?}", slice1);
    println!("Slice: {:?}", slice2); 
  }