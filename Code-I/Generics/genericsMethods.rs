#[derive(Debug)]

struct Point<T>{
    x: T,
    y: T,
}

impl <T> Point<T>{
    fn hello(&self){
        println!("Hello");
    }
}

fn main(){
    let p1: Point<i32,i32> = Point{x: 5, y:10};
    

 }