fn main() {
    float_sizes();
    integers();
    integers_sizes();
    let multiply_awnser: i64 = multiply(100, 10);
    let devide_awnser: f64 = divide(10, 8);
    println!(
        "multiply_awnser:{} ~ devide_awnser:{}",
        multiply_awnser, devide_awnser
    )
}

// Integers

fn float_sizes() {
    let x: f64 = 10.0 / 3.0;
    // f64 has 64 bits(8bytes) of storage
    // 3.333333...
    let y: f32 = 10.0 / 3.0;
    // f32 has 32 bits(4bytes) of storage
    println!("x:{}, y:{}", x, y);
}

fn integers() {
    let minety: i32 = 90;
    let negative_fve: i32 = -5;
    let one_thousand: i64 = 1_000;
    let exactly_three: i32 = 10 / 3;
    // let this_will_panic: i32 = 5 / 0; // kaboom rsrsrs!

    println!(
        "minety:{}, negative_fve:{}, one_thousand:{} , exactly_three:{}",
        minety, negative_fve, one_thousand, exactly_three
    );
}

fn integers_sizes() {
    /*
    i8 = 8 bits(1B) ~ -127 to 128
    i16 = 16 bits(2B)  ~ -32,768 to 32,767
    i32 = 32 bits(4B)
    i64 = 64 bits(8B)
    i128 = 128 bits(16B)
     */
    /*
    unsigned Integers:
    u8 = 0 to 255
    u16 =0 to 65,535
    u32 =0 to 4,294,967,295
    u64 = ......
    u128 =.....
     */
    let minety: i32 = 90;
    let negative_fve: i32 = -5;
    let one_thousand: i64 = 1_000;
    let exactly_three: i32 = 10 / 3;
    // let this_will_panic: i32 = 5 / 0; // kaboom rsrsrs!

    println!(
        "minety:{}, negative_fve:{}, one_thousand:{} , exactly_three:{}",
        minety, negative_fve, one_thousand, exactly_three
    );
}

fn multiply(x: i64, y: u8) -> i64 {
    return x * (y as i64);
}
fn divide(x: i32, y: u16) -> f64 {
    return x as f64 / y as f64;
}
