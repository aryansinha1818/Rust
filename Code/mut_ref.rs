fn main(){
let mut s = String::from("hello");
func_call(&mut s);
println!("{}", s);

fn func_call(str1: &mut String){
    str1.push_str(", world");
}
}