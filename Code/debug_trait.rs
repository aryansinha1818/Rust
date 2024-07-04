

// std::fmt::Debug;
// std::fmt::Display;


struct Val{
    val1 : i32,
    val2 : String
}

impl std::fmt::Debug for Val{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{}", self.0)
    }
}

fn main(){
    let v1 = Val {val1: 9, val2: "the name".to_string()};
    println!("{:#?}", v1);
}