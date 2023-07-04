fn main() {
    // step 1:  stack [17, 18]
    increment_and_descrement(17, 18);
    // step 4:  stack []

    //-----
    // step 1:  stack [x, y, 20]
    let [x, y] = return_double_value(20);
    // step 4:  stack []
    println!("value_one: {}, value_two: {}", x, y);
}
// step 2:  stack [17, 18] - get(stack.len - 1) and get(stack.len - 2)
fn increment_and_descrement(x: u16, y: u16) -> () {
    print_all_values(x + 1, y - 1); // step 3:  stack [17, 18, 18, 17] - add params value
}
// step 3:  stack [17, 18, 18, 17] - get(stack.len - 1) and get(stack.len - 2)
fn print_all_values(x: u16, y: u16) -> () {
    println!("incremented: {}, descremented: {}", x, y);
}
// -----------------------------------
// step 2:  stack [x, y, 20] - get(stack.len -1)
fn return_double_value(num: u32) -> [u32; 2] {
    return [num + 1, num + 2]; // step 3:  stack [21, 22, 20]
}
