#[derive(Debug)]
struct Point<T,U>{
    x: T,
    y: U
}

fn main(){
    let a: Point<i32, String> = Point{x: 9, y: String::from("Aryan")};
    println!("{:?}",a);
}