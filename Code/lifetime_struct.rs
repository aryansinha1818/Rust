#[derive(Debug)]
struct Borrowed<'a>(&'a i32);

#[derive(Debug)]
struct Own<'a>(&'a i32);

#[derive(Debug)]
struct Own1<'a>{
    x: &'a i32,
    y: &'a str
}

#[derive(Debug)]
struct Name1<'a>{
    p: &'a i32,
    q: &'a i32
}

#[derive(Debug)]
enum Either<'a>{
    NUm(i32),
    Num1(&'a i32)
}

fn main(){
    let x = 18;
    let y = 9;

    let fun1 = Own(&x);
    println!("the value of Own struct is {:?}", fun1);

    let fun2 = Name1{q: &x, p :&y};
    println!("The value is {:?}", fun2);

    let fun3 = Either::NUm(x);
    let fun4 = Either::Num1(&y);
    println!("The value is enum variant 1 {:?}", fun3);
    println!("The value is {:?} enum variant 2", fun4);

    println!("Success");
}