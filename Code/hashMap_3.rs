use std::collections::HashMap;
fn main(){
    let mut map1: HashMap<i32,i32> = HashMap::with_capacity(100);
    let c1 = map1.capacity();
    println!("{}",c1);
    map1.shrink_to(50);
    let c2 = map1.capacity();
    println!("the value of c2 is {}", c2);

}