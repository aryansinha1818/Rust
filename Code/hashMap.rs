use std::collections::HashMap;

fn main(){
    let teams: [(&str, i32); 3] = [
        ("abc", 1),
        ("pqr", 2),
        ("fgh", 3)
    ];
    
    let mut map1 = HashMap::new(); 
    for it in &teams{
        map1.insert(it.0, it.1);
    }

    let map2: HashMap<&str, i32> = HashMap::from(teams);
}