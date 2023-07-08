fn main() {
    let string: String = String::from("Hello World");
    let slice: &str = &string[0..5];

    let years: Vec<i64> = vec![1999, 2004, 2009, 2014];

    let two_first_years: &[i64] = &years[0..2];

    println!(
        "string_slice: {} and two_first_years: {:?}",
        slice, two_first_years
    );
}
