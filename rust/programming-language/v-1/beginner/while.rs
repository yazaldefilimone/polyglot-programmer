fn main() {
    let mut num: i32 = 0;
    while num != 5 {
        println!("{}", num);
        num = num + 1;
    }
    println!("LIFTOFF")
}

fn while_test_2() {
    let elements: [i32; 5] = [10, 20, 30, 40, 50];

    let mut index: usize = 0;

    while index < 5 {
        println!("index: {}, value: {}", index, elements[index]);
        index = index + 1
    }
}
