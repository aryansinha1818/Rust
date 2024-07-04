fn main(){
    let a = [1,2,3,4,5];

    for(i,v) in a.iter().enumerate(){
        
        print!("The {}th element is {}", i+1, v);
    }
}