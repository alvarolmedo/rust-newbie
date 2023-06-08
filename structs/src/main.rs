use std::io;

struct User{
    active: bool,
    username: String,
    email: String,
    age: u32,
}
fn main() {
    println!("Give you the first steps wth the structs...");
 
    let user1 = User {
        active: true,
        username: String::from("Alvaro"),
        email: String::from("Alvaro@alvaro.com"),
        age: 4
    };

    let mut user2 = User {
        active: true,
        username: String::from("Alvaro"),
        email: String::from("Alvaro@alvaro.com"),
        age: 4
    };

    //mut the entire struct not only one field :(((
    user2.email = String::from("otroemail@gmail.com");
    user2.username = String::from("otro");

    //short
    let mut user_fast = User {
        active: false,
        ..user1
    };
        
}

