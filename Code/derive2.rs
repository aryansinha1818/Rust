#[derive(Debug)]
struct Val{
    val1 : i32,
    val2 : String
}
fn main(){
    let v1 = Val {val1: 9, val2: "the name".to_string()};
    println!("{:#?}", v1);
}