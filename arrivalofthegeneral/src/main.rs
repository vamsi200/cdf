fn main() {
    let mut input = String::new();
    let mut values = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();
    std::io::stdin().read_line(&mut values).unwrap();
    let parsed_values: Vec<i32> = values
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let max = *parsed_values.iter().max().unwrap();
    let max_index = parsed_values.iter().position(|&x| x == max).unwrap();

    let min = *parsed_values.iter().min().unwrap();
    let min_index = parsed_values.iter().rposition(|&x| x == min).unwrap();

    let mut swap = max_index + (n - 1 - min_index);
    if max_index > min_index {
        swap -= 1;
    }

    println!("{}", swap);
}
