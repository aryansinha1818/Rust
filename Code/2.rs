use std::io;
use rand::Rng;

fn main(){
    println!("Guess the number");

    let secret_no = rand::thread_rng().gen_range;

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");
    println!("You guessed: {}", secret_no);
 }