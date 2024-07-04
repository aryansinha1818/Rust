fn main(){
    value(Coin::Quarter(IndiaState::Rajasthan));
 }

#[derive(Debug)]
enum IndiaState{
    Dehradun,
    Delhi,
    Punjab,
    Rajasthan,
    
}

enum Coin{
    Penny, 
    Nickel, 
    Dime, 
    Quarter(IndiaState),
}

fn value(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => {
            println!("Hellllllllllllllllsjsdjvn");
            1
        },
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("{:?}",state);
            25
        }
    }
}