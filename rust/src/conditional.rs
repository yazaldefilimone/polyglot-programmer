fn main() {
    let number1: i32 = 20;
    let number2: i32 = 10;

    if number1 > number1 {
        println!("{} > {}", number1, number2)
    } else {
        println!("{} <= {}", number1, number2)
    }

    let mut result: i32 = 0;
    let num1: i32 = 0;
    let num2: i32 = 1;
    let num3: i32 = 2;
    let booleane: bool = true;

    if booleane && num1 > num2 {
        result = num1;
    } else if num2 > num3 {
        result = num2;
    } else {
        result = num3;
    }
    println!("{}", result);
}
