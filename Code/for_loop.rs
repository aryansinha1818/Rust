fn main(){
    let arr: [i32; 5] = [1,2,3,4,5];
    for itr in arr{
        println!("{}", itr);
    }
    println!(" ");
    let arr2: [f64; 3] =  [2.2, 2.4,2.6];
    for itr1 in &arr2{
        println!("{}", *itr1);
    }
    println!("{:?}", arr);
    println!("{:?}", arr2);

    let _arr2 = [10,20,30];
    println!("Success");
}