fn exemple_one() {
    {
        let s = "text"; // s= is valid
                        // ...
        println!("{}", s)
    } // s = is not valid
}

fn shallow_copy_and_deep_copy() {
    let string_p: i32 = "Hello, world"; // stack
    let string_one = String::from("text"); // heap
    let string_two = string_one;

    // println!("{}", string_one); // Error
    println!("{}", string_two); // Done = text!
    println!("{}", string_p); // Done = text!
}

fn string_worked() {
    let mut message: String = String::from("Hello");
    message.push_str(", World");

    println!("{}", message) // Hello, World
}

fn copy_string() {
    // heap
    let mut message: String = String::from("Hello");
    message.push_str(", World");
    let secund_message: String = message; // message: no valid more

    let three_message: String = secund_message.clone(); // copy data in heap

    println!(
        "secund_message:'{}' and three_message:'{}'",
        secund_message, three_message
    );

    // stack

    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
}

fn main() {
    {
        // s não é válida aqui, ainda não está declarada
        let s = "text"; // s é válida deste ponto em diante
                        // faz alguma coisa com s
        println!("{}", s)
    } // agora este escopo terminou, e s não é mais válida
}
