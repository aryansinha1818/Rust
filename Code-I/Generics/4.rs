struct Point<T>{
    x: T,
    y: T,
}

impl<T> Point<U>{
    fn x(&self) -> &U{
        &self.x
    }
}

fn main(){
    let p = Point{x: 5, y:10};
    
}