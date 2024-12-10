use std::collections::HashMap;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut input_parsed: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let original_length = input_parsed.len();

    let mut temp: HashMap<i32, bool> = HashMap::new();
    input_parsed.retain(|&x| temp.insert(x, true).is_none());

    match original_length.cmp(&input_parsed.len()) {
        std::cmp::Ordering::Greater => {
            println!("{}", original_length - input_parsed.len());
        }
        std::cmp::Ordering::Less => println!("{}", input_parsed.len()),
        std::cmp::Ordering::Equal => {
            println!("{}", original_length - input_parsed.len());
        }
    }
}
