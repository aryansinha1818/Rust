// pub fn notify(item: &impl Summary){
//     println!("Breaking news! {}", item.summarize());
// }

pub fn notify<T: Summary>(item : &T){
    println!("Breaking news! {}", item.summarize());
}

pub fn notify(item1 : &impl Summary, item2: &impl Summary){

}

pub fn notify<T: Summary>(item1: &T, item2: &T){
    
}