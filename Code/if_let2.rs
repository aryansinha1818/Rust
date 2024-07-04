fn main(){
    let a: Option<i32> = Some(9);
    if let Some(i) = a{
        println!("Success");
    }
}