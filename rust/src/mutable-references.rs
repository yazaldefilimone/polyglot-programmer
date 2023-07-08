fn main() {
    let year: i64 = 2023;
    let borrow_year: &i64 = &year;
    // let mut_borrow_year: &mut i64 = &mut year; // error
    let mut next_year: i64 = year + 1;
    let mut_borrow_next_year: &mut i64 = &mut next_year;
    // let borrow_next_year: &i64 = &next_year; //error

    println!(
        "year: {} and borrow_year: {} and mut_borrow_next_year: {}",
        year, borrow_year, mut_borrow_next_year
    );
}
