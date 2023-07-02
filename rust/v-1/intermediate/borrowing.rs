fn main() {
    let mut name: String = String::from("Jhoe");
    // incorrect_change_name(&name);
    corrent_change_name(&mut name);
    print!("\n");
    print!("{}", name);
    print!("\n");
    print!("{}", return_borrowing_value());
}

// fn incorrect_change_name(name: &String) { //error
//     print!("{}", name);
//     name.push_str("Doe");
// }
fn corrent_change_name(name: &mut String) {
    print!("{}", name);
    name.push_str(" Doe");
}

// fn return_borrowing_value() -> &String { //error
//     let value: String = String::from("borrowing");
//     &value
// }

//no error
fn return_borrowing_value() -> String {
    let value: String = String::from("borrowing");
    value
}
