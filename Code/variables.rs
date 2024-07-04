fn main(){
    let mut x = 5;

    println!("{}", x);
    x = 6;
    println!("{}", x);

    //Shadowing
    let y = 50;
    println!("{}", y);
    let y = 60;
    println!("{}", y);
 }