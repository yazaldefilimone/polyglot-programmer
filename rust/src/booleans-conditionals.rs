fn main() {
    let should_we_go_fast: bool = true;
    let should_we_go_slow: bool = false;

    println!(
        "should_we_go_fast:{}, should_we_go_slow:{}",
        should_we_go_fast, should_we_go_slow
    );

    println!(
        "with as should_we_go_fast:{}, should_we_go_slow:{}",
        should_we_go_fast as u8, should_we_go_slow as u8
    );
    conditionals(1); // Need more cats!
    conditionals(2); // Multiple cats!
}

fn conditionals(cats: i32) {
    if cats > 1_00 {
        println!("To many cats!")
    } else if cats > 1 {
        println!("Multiple cats!")
    } else {
        println!("Need more cats!")
    }
}
