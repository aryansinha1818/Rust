enum Home_Decor{
    List1,
    List2,
    List3
}

fn main(){
    let l1 = Home_Decor::List1;
    let val = call(l1);
    println!("{}", val);
}

fn call(take: Home_Decor) -> i32{
    match take{
        Home_Decor::List1 => 9,
        Home_Decor::List2 => 99,
        Home_Decor::List3 => 999,
    }
}
