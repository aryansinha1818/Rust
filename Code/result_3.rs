
fn main(){
    let maybe_number = Some(5);

    let transformed_number = maybe_number.map(|x| x*2);

    match transformed_number{
        Some(value) => println!("Transformed 
        value: {}", value),
        None => println!("No value present"),
    }
}