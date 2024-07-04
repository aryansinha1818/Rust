trait Word{
    fn spell(&self) -> String;
}

struct Book;

impl Word for Book{
    fn spell(&self) -> String{
        //define here
        println!("Hey this is my work");
        String::from("This is the return value")
    }
}

fn main(){
    // in a variable took one instance of Book struct
    let b1 = Book;
    let res = b1.spell();

    println!("{}", res);
}