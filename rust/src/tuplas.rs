fn main() {
    let list: (i64, i64, i64) = (10, 1, 0);
    println!("{:?}", list);
    println!("{:?}", list.0);
    let (x, y, z) = list;
    println!("x:{}, y:{}, z:{}", x, y, z);
}
