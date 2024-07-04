fn main(){
    let s1 = String::from("Hello");
    name(&s1);
}

fn name(s: &str){
    println!("{}", s);
}