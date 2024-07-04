use std::collections::HashMap;

fn main(){
    let mut map1: HashMap<&str, i32> = HashMap::new();

    //name will be inserted with 100 as value and name as key
    map1.entry("name").or_insert(100);


    map1.entry("name1").or_insert_with(random_stuff);

    let mut check = map1["name"];
    assert_eq!(check,100);
    println!("Success");

}

fn random_stuff() -> i32{
    400
}