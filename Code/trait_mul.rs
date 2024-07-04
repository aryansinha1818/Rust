use std::ops;

fn multiply< T: std::ops::Mul<Output = T>>(a: T, b: T)->T{
    a*b 
}

fn main(){
    assert_eq!(6, multiply(2u8, 3u8));
    assert_eq!(5.0, multiply(1.0, 5.0));

    let val1 = multiply(9u8, 9u8);
    println!("{}", val1);

    println!("Success");
}

// fn multiply<T: std::ops::Mul<Output = T>>(a: T, b: T)->T{

// }