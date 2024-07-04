fn main(){
    enum Option<T>{
        Some(T),
        None,
    }

    //Examples
    let value: Option<i32> = Some(9);
    let str1: Option<String> = Some("Hello 01010");
    let absent: Option<i32> = None;
 }