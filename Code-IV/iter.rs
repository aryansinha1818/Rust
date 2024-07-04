fn main(){
    let mut v1 =vec![1,2,3];
    
    let mut p = v1.iter_mut();

    if let Some(v) = p.next(){
        *v = 0;
    }
    for i in v1 {
        println!("{}", i);
    }
}