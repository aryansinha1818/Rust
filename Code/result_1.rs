use std::fs::File;
use std::io::ErrorKind;

fn main(){
    let f = File::open("hello.txt");

    let f = match f{
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}