enum Num{
    one, //0
    two, //1
    three
}

enum Num2{
    one =1,
    two, 
    three
}

fn main(){
//using as u8 we can print it
    assert_eq!(Num::two as u8,Num2::one as u8);
    println!("Success");
}