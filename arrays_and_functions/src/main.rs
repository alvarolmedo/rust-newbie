use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    // If intro a index number print this element and its info
    if index < a.len() {
        let element = a[index];

        println!(
            "The value of the element at index {} is: {}",
            index, element
        );
        my_other_function(element);
        let element_plus_1 = add_one(element);
        let element_plus_2 = add_two(element);
        println!("The next numbers are: {} and {}", element_plus_1, element_plus_2);
    // If intro the same than lenght, print all elements
    } else if index == a.len() {
        for element in a {
            println!("The values is: {}", element);
        }   
    } else { 
        println!("Index entered ({}) is greater than array length ({})",index, a.len());
    }
}

fn my_other_function(number: u32) {
    println!("Another function, same number: {}", number);
}

fn add_one(number: u32) -> u32 {
    number + 1 //no termina con punto y coma
}

fn add_two(number: u32) -> u32 {
    return number + 2; //termina con punto y coma, hay que utilizar return
}
