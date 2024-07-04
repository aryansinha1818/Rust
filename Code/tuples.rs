#[allow(unused_variables)]
fn main(){
    let tup1: () = ();
    let tup2: (i32, f64, &str) = (9, 9.9, "Space");
    println!(" {} {} {} ", tup2.0, tup2.1, tup2.2);
    let (x,y);
    (..,y) = (1,2);
    [x,..] = [3,4];
    assert_eq!([x,y], [3,2]);
    println!("Success!");
}