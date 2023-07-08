fn main() {
    let numbers: Vec<i64> = vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24];

    let (sum_of_nums, param_sum) = sum(numbers); // or numbers.clone()
    let (product_of_nums, params_product) = product(param_sum); // or numbers.clone()
    let average_of_nums: i64 = average(params_product);

    println!("Sum of these numbers: {}", sum_of_nums);
    println!("Product of these numbers: {}", product_of_nums);
    println!("Average of these numbers: {}", average_of_nums);
}

fn sum(numbers: Vec<i64>) -> (i64, Vec<i64>) {
    let mut total: i64 = 0;

    for num in numbers.iter() {
        total += num;
    }

    (total, numbers)
}

fn product(numbers: Vec<i64>) -> (i64, Vec<i64>) {
    let mut total: i64 = 1;

    for num in numbers.iter() {
        total *= num;
    }

    (total, numbers)
}

fn average(numbers: Vec<i64>) -> i64 {
    let length: i64 = numbers.len() as i64;
    let (num, _) = sum(numbers);
    num / length
}
