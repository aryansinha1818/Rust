fn main(){
    let (x,y) = sum_multi((2,3));

    assert_eq!(x,5);
    assert_eq!(y, 6);

    println!("Success");
}

fn sum_multi(nums: (i32, i32)) -> (i32, i32){
    (nums.0 + nums.1, nums.0 * nums.1)
}