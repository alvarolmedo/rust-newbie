use std::io;

fn main() {
    println!("Give you the first word in the phrase...");

    println!("Please input your phrase.");

    let mut phrase = String::new();

    io::stdin()
        .read_line(&mut phrase)
        .expect("Failed to read line");

    println!("Your phrase: {}", phrase);

    println!("The first phrase is: {}",first_word(&phrase));
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
