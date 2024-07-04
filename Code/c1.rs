//Closures

user std::thread;
use std::time::Duration;

fn simulated_expense(intensity: u32) -> u32{
    println!("calc slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn main(){
    let simulated_intensity
}