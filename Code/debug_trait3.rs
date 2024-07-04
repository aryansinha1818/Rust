struct P{
    x: i32,
    y: f64
}

fn main(){
    let point: P = P{x: 9, y: 9.9};
    assert_eq!(format!("{}", point), "Display: 3.3 + 7.2i");
    assert_eq!(format!"{:?}", point), "Debug: Complex {real: 3.3, imag: 7.2i}");

    println!("{}", point);
    println!("{:?}", point);

    println!("Success");
}