fn main() {
    let mut input = String::new();
    let mut values = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input_parsed: i32 = input.trim().parse().unwrap();

    std::io::stdin().read_line(&mut values).unwrap();
    let values_parsed: Vec<i32> = values
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    for i in 1..input_parsed + 1 {
        let position = values_parsed.iter().position(|&x| x == i);

        match position {
            Some(index) => print!("{}", (index + 1)),
            None => print!("ntg"),
        }
        if i != input_parsed {
            print!(" ");
        }
    }
}
