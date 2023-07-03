fn main() {
    let str_or_none: Option<String> = string_or_none(false);
    let str_or_none_test: Option<String> = string_or_none(true);
    println!("{:?}", str_or_none);
    println!("{:#?}", str_or_none_test);

    let test: Result<64, String> = result_test(true);

    if test::OK {
        println!("Success: {:?}", test);
    }

    if test::Err {
        println!("Error Message: {:?}", test);
    }
}

fn string_or_none(params: bool) -> Option<String> {
    if params {
        let str: String = String::from("Hei I'am string!");
        return Some(str);
    }
    return None;
}

fn result_test(param: bool) -> Result<i64, &'static str> {
    if param {
        Ok(100)
    } else {
        Err("Some Error")
    }
}
