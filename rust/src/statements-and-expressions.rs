fn main() {
    let x: f64 = 1.0;
    let y: f64 = 20.0;
    let result: f64 = multiply_both(x, y);
    println!("{}", result);
    println!("{}", conditionals(10))
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

fn conditionals(cats: i32) -> String {
    let message = if cats > 1_00 {
        "To many cats!"
    } else if cats > 1 {
        "Multiple cats!"
    } else {
        "Need more cats!"
    };

    message
}
