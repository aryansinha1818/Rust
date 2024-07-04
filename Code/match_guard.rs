fn main(){
    let num: Option<i32> = Some(9);
    let split = 5;
    match num {
        Some(i) if i<split => assert!(i < split),
        Some(i)  => assert!(i>= split),
        None => (),
    }

    println!("Success!");
}