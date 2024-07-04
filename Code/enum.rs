enum Name{
    Val1(String),
    Val2(i32)
}

fn main(){
    let num = Name::Val1(String::from("Hello World!"));
    let num2 = Name::Val2(9);
    
}