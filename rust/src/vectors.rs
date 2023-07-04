fn main() {
    vectors();
    arrays();
}

fn vectors() -> () {
    let mut numbers: Vec<i8> = vec![1, 2, 3, 4, 5];
    let mut store_numbers: Vec<i8> = Vec::new();
    for num in &numbers {
        if *num < 5 {
            store_numbers.push(*num + 1);
        }
        println!("num: {}", num);
    }
    println!("length: {}", numbers.len());
    numbers.extend(store_numbers);
    println!("length: {}", numbers.len());
    let last: Option<i8> = numbers.pop();
    println!("length: {:?}, {}", last, numbers[numbers.len() - 1]);
}
fn arrays() -> () {
    let numbers: [i64; 5] = [1, 2, 3, 4, 5];

    for num in numbers {
        println!("arr_num: {}", num);
    }
}
