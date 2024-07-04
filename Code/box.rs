//when we declare something in the heap memory
// we access that element using dereference operator.
fn main(){
    let x: Box<i32> = Box::new(9) ;
    assert_eq!(*x,9);
    println!("Success");
}