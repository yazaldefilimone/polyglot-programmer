fn main() {
    let string_no_copy: String = String::from("texto"); // 'string_no_copy': enter in scope.

    cat_value(string_no_copy); // move value in funcion 'cat_value'
                               // ... 'string_no_copy' is not valid

    let inter_copy: i32 = 5; // 'inter_copy' enter in scope

    make_copy(inter_copy); // 'inter_copy' is copied in  funcion make_copy, 'inter_copy' is a type 'i32' == 'copy', continue valid in scope
                           // teste(inter_copy, string_no_copy) //string_no_copy is moved
} // here all variable without in scope but `string_no_copy` is moved no called 'drop'

fn cat_value(string_no_copy: String) {
    // `string_no_copy` enter in scope.
    println!("{}", string_no_copy);
} //  here `string_no_copy` is without in scope and  method 'drop' is called

fn make_copy(inter_copy: i32) {
    // inter_copy enter scope
    println!("{}", inter_copy);
} // here a value inter_copy:i32 without scope

// fn teste(inter: i32, string_no_copy: String) {
//     println!("i`m inter {} and i`m string: {}", inter, string_no_copy);
// }
