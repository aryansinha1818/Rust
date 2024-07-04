

fn main(){
        let s1: &str = "Hello World!";
        let s2: String = String::from("Hello World 2!!");

        //to_string and to_owned are similar
        let s3 = "Hello world 3!!".to_string();
        let s4 = "Hi".to_owned();

        // .. will take the entire string
        let s5: &str = &s4[..];

        println!("{} {} {} {} {}", s1, s2, s3, s4, s5);
}