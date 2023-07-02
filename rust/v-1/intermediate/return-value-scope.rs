fn main() {
    let receive_value: String = deviver_value();
    let new_value: String = String::from("Hi There!!");
    let returned: String = recive_and_return(new_value);

    println!("received:{}, and returned:{}", receive_value, returned);

    let (st, size): (String, usize) = return_size(returned);

    println!("string:{}, length:{}", st, size)
}

fn deviver_value() -> String {
    let value: String = String::from("Hi, a message here");
    value
}

fn recive_and_return(value: String) -> String {
    return value;
}

fn return_size(value: String) -> (String, usize) {
    let size: usize = value.len();

    (value, size)
}
