fn main(){
    let b: Box<i32> = Box::new(5) ; // new keyword --> What is the value that needs to be in the heap.
    // In the stack we have an address which says there is something in the heap. In the 
    // stack we have the pointer to the heap. 
    println!("b = {}", b);

    let a = Box::new(9);
    println!("a = {}", a); // after main the a value will be de-alocated
 }