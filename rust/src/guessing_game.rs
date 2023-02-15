use std::io;

pub(crate) fn game() {
    println!("Guessing the number!");
    println!("Enter width a number how do you think.");

    let mut user_number = String::new();

    io::stdin()
        .read_line(&mut user_number)
        .expect("Error to read input");

    println!("You say: {}", user_number);
}
