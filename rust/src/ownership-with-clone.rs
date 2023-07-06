fn main() {
    let years: Vec<i64> = get_years(); // take to owenership
    print_years(years);
    println!("------------ second ---------------");
    print_years(years_one); // return year and in end scope its dealloc(years)
} //dealloc(years)

fn get_years() -> Vec<i64> {
    let years: Vec<i64> = vec![1999, 2004, 2009, 2014]; // alloc
    return years; // transfrer to owenership to main ("move""  'years' to 'main' scop)
}

fn print_years(years: Vec<i64>) -> Vec<i64> {
    for year in years.iter() {
        println!("Year: {}", year);
    }
    return years;
}
