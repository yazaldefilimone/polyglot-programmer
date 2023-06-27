fn main() {
    println!("Hello, World");
    let greetings = "Hello";
    let object = "World";
    println!("{}, {}", greetings, object);

    let x = format!("Hello, {}", object);
    println!("{}", x);
    panic!("{}", x);
    // not executed
}
