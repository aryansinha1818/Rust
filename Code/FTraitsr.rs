trait Animal {
    fn sound(&self) -> String{
        String::from(("Hello Planet"))
    }
}

struct Sheep;
struct Fish;

impl Animal for Sheep{
    fn sound(&self) -> String{
        String::from("I eat grass")
    }
}

impl Animal for Fish{
    fn sound(&self) -> String{
        String::from("I live in water")
    }
}

fn main(){
    let anml1 = Sheep;
    let anml2 = Fish;
    println!("Hi I am Sheep and {}", anml1.sound());
    println!("Hi I am Fish and {}", anml2.sound());
}