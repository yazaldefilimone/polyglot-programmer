fn main() {
    let elements: [i32; 5] = [10, 20, 30, 40, 50];

    for element in elements.iter() {
        println!("{}", element)
    }
    rev()
}

fn rev() {
    for numero in (1..4).rev() {
        println!("{}!", numero);
    }
    println!("LIFTOFF!!!");
}
