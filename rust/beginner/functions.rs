fn main() {
    println!("Hello, world!");
    let x: i32 = 33;

    other_function();
    println!("{}", funcion_with_param(x, 54));
}

fn other_function() {
    println!("Other Funcion.");
}

fn funcion_with_param(x: i32, y: i32) -> i32 {
    return x + y;
}
