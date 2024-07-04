trait Animal{
    fn sound(&self) -> String;
};

struct Sheep;

impl Animal for Sheep{
    fn sound(&self) -> String{
        String::from("Hello World!");
    }
}

fn main(){
    let sheep = Sheep;
    let sound = sheep.sound();

    println!("{}", sound);
}