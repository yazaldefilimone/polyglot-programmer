fn main() {
    let x: f64 = 1.0;
    let y: f64 = 20.0;
    let result: f64 = multiply_both(x, y);
    println!("{}", result)
}

//NB: if a functiona ends with an expression it automantically returns that expression

fn multiply_both(x: f64, y: f64) -> f64 {
    // return x * y; // expression
    /*
       return x * y; = statements
       x * y = expression
    */

    x * y // automantically return
}
