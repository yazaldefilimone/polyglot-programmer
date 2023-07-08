fn main() {
    let years: Vec<i64> = get_years();
    print_years(&years);
    println!("------------ second ---------------");
    print_years(&years);
}

fn get_years() -> Vec<i64> {
    let years: Vec<i64> = vec![1999, 2004, 2009, 2014];
    return years;
}

fn print_years(years: &Vec<i64>) -> () {
    for year in years.iter() {
        println!("Year: {}", year);
    }
}
