fn main() {
    // let list: (i64, i64, i64) = (10, 1, 0);
    // println!("{:?}", list);
    // println!("{:?}", list.0);
    // let (x, y, z) = list;
    // println!("x:{}, y:{}, z:{}", x, y, z);
    structs();
    println!("------array-----");
    arrays();
}

fn structs() -> () {
    struct Point {
        x: i64,
        y: i64,
    }

    let mut point = Point { x: 19, y: 1 };
    let Point { x, .. } = point;
    println!("x:{}", x);
    println!("y:{}", point.y);
    point.x = 102;
    println!("x:{}", point.x);
}

fn arrays() -> () {
    let years: [i64; 4] = [2001, 2002, 2003, 2004];
    // let first_year = years[0];
    // println!("{}", first_year);
    // let [_, second, three, ..] = years;
    for year in years.iter() {
        println!("year: {}", year);
    }
}
