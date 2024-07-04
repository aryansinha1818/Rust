fn main(){
    let mut s: String = String::from("foo");

    //mut
    s.push_str(" bar");
    println!("{}", s);


    // range: ...., replace_with: "string"
    // .. tells we need the entire string to be replaced
    //s.replace_range(range: .., replace_with: "baz");
    s.replace_range(.., "baz");
    println!("{}", s);

}