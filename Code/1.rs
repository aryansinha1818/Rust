#[derive(Debug)]

struct Point<T, U>{
    x: T,
    y: U,
}

fn main(){
    let p1: Point<i32,i32> = Point{x: 5, y:10};
    let p2: Point<f64, f64> = Point{x: 5.5, y:10.10};
    let p3 = Point{x:10, y:25.2};
    println!("{:?}", p1);
    println!("{:?}", p2);

 }