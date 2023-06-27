fn main() {
    let x = 1.4;

    let y = 2.2;
    println!("{} * {} is equal {}", x, y, x + y);
    mutabily();
}

fn mutabily() {
    let mut x = 10.5;
    x = 1.0;
    println!("{}", x)
}
