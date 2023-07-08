fn main() {
    let string: String = "Hello World";
    let slice: &str = &string[0..4];
    println!("{}", slice)
}
