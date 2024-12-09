fn main() {
    let mut input = String::new();
    let mut values = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let parsed_input: i32 = input.trim().parse().unwrap();
    std::io::stdin().read_line(&mut values).unwrap();
    let p: Vec<i32> = values
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let total: i32 = p.iter().sum();
    let average: f64 = total as f64 / parsed_input as f64;

    println!("{:.6}", average);
}
