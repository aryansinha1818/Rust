fn plus_one(val : Option<i32>)-> Option<i32>{
    match val{
        Some(i) => Some(i+9),
        None => None 
    }
}

fn main(){
    let num = Some(9);
    let num2 = plus_one(num);
    let none = plus_one(None);

    println!("Success");
}