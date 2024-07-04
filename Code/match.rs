enum Val{
    Item1,
    Item2,
    Item9
}

fn grocery(serial_number: Val) -> u8{
    match serial_number{
        Val::Item1 => 9,
        Val::Item2 => 18,
        Val::Item9 => 27,
    }
}

fn main(){

}