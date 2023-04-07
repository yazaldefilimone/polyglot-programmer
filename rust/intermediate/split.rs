fn main() {
    let sentence: String = String::from("Hello Workd");
    let frist: &str = frist_sentence(&sentence);
    print!("{}", frist)
}

fn frist_sentence(sentence: &String) -> &str {
    let bytes: &[u8] = sentence.as_bytes();
    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &sentence[0..index];
        }
    }
    return &sentence[..];
}
