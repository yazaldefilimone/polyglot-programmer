fn main() {
    let numbers: Vec<i64> = vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24];
    let sum_of_nums = sum(&numbers);
    let product_of_nums = product(&numbers);
    let average_of_nums = average(&numbers);

    println!("Sum of these numbers: {}", sum_of_nums);
    println!("Product of these numbers: {}", product_of_nums);
    println!("Average of these numbers: {}", average_of_nums);

    let other_numbers = vec![1, 2, 3, 4, 5, 6];
    let (slice1, slice2) = first_three(&numbers, &other_numbers);

    println!("The first three elements in `slice1` are:");

    for num in slice1 {
        println!("• {}", num);
    }

    println!("The first three elements in `slice2` are:");

    for num in slice2 {
        println!("• {}", num);
    }
}

fn sum(numbers: &[i64]) -> i64 {
    let mut total = 0;

    for num in numbers.iter() {
        total += num;
    }

    total
}

fn product(numbers: &[i64]) -> i64 {
    let mut total = 1;

    for num in numbers.iter() {
        total *= num;
    }

    total
}

fn average(numbers: &[i64]) -> i64 {
    let length = numbers.len() as i64;

    sum(numbers) / length
}

fn first_three<'life, 'time>(
    numbers1: &'life Vec<i64>,
    numbers2: &'time Vec<i64>,
) -> (&'life [i64], &'time [i64]) {
    let first_num_one: &'life [i64] = &numbers1[0..3];
    let first_num_two: &'time [i64] = &numbers2[0..3];
    (first_num_one, first_num_two)
}
