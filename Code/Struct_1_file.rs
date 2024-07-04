struct User{
    username: String,
    email: String,
    sign_in: u64,
    active: bool,
}



fn main(){
    let mut user1: User = User {
        email: String::from("djvnj@gmail.com"),
        username: String::from("Aryan"),
        sign_in: 10,
        active: true,
    };

    let name = user1.username;
    user1.username = String::from("Aryan Sinha");

    let user2 = build_user(String::from("nvf@gmail.com"), String::from("Kittu") );
 }

 //Below whatever we will pass in the argument we will get that
 fn build_user(email: String, username: String) -> User{
    User{
        email: email,
        username,
        active: true,
        sign_in: 100,
    }
 }