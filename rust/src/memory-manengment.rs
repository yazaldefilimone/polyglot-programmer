fn main() {
    let sum: u64 = final_order();
    println!("{}", sum);
}

fn final_order() -> u64 {
    let mut sum: i64 = 0;

    {
        let numbers: Vec<i64> = vec![1, 2, 3, 4, 5]; // alloc

        for number in numbers.iter() {
            sum += number;
        }
    } // dealloc

    return finish(sum);
}

fn finish(sum: i64) -> u64 {
    return sum as u64 * 2;
}
