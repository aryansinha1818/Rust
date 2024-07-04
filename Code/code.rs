use std::fs::File;
use std::io::ErrorKind; //this will help us to define the error and not panic.

fn main(){
 let f: Result<File, Error>  = File::open("hello.txt");

 let f: File = match f{
    ok(file: File) => file,
    Err(error: Error) => match error.kind(){
        
    }
 }
}