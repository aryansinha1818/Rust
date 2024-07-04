#[derive(Debug)]

struct Rectangle{
    width: i32,
    height: i32,
}

//to implement constructors/function we use the impl block
impl Rectangle {
    fn area(&self) -> i32{
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle)->bool{
        self.width > other.width && self.height > other.height
    }
}

fn main(){

        
        let rect1 = Rectangle{
            width: 10,
            height: 20,
        };

        println!("rect: {:?}", rect1);

        println!(
            "The area of the rect is {} ",
            rect1.area()
        );

 }