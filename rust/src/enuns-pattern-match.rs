#[derive(Debug)] // Implementa a trait Debug para a enumeração colo
enum color {
    green,
    yellow,
    white,
}

fn main() {
    let color_green: color = color::green;
    println!("{:?}", color_green);
    let match_result = pattern_match(color_green);
    println!("color math is: {}", match_result);
}

// Pattern match
fn pattern_match(colorType: color) -> &'static str {
    match colorType {
        color::green => "color is green",
        color::yellow => "color is yellow",
        color::white => "color is white",
        _ => "unknow that color",
    }
}
