fn main(){
    let a = [1,2,3];
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    {
        let v2 = vec![4,5,6];
    }


    //access elements
    let v3:Vec<i32> = Vec::new();
    let v4 = vec![10,20,30,40,50];
    //using reference
    let third = &v4[2];
    println!("{}", third);

     
}