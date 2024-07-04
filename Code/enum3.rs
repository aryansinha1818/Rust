enum Message{
    Quit,
    Move{x: i32, y: i32},
    Write(String)
}

enum Bat{
    Bat1{c: i32, d: String},
    Bat2(i32),
}

fn main(){
    let v1 = Message::Move{x: 1, y: 10};
    let v2 = Message::Write(String::from("Hi I am Rusty"));
    let v3 = Bat::Bat1{c: 1, d: String::from("Hello Rust")};
    

    println!("Success");
}