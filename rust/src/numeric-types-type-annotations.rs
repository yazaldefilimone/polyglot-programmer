fn main() {
    let answeser: f64 = multiply_both(1.1, 2.2);
    println!("{}", answeser)
}

// Type annotation

fn multiply_both(x: f64, y: f64) -> f64 {
    return x * y;
}
