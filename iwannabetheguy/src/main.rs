fn main() {
    let mut input = String::new();
    let mut x = String::new();
    let mut y = String::new();

    std::io::stdin().read_line(&mut input).unwrap();
    let input: i32 = input.trim().parse().unwrap();
    std::io::stdin().read_line(&mut x).unwrap();
    std::io::stdin().read_line(&mut y).unwrap();

    let x_parsed: Vec<i32> = x
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();
    let y_parsed: Vec<i32> = y
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();
    let mut all_levels = x_parsed.clone();
    all_levels.extend(y_parsed);
    all_levels.sort();
    all_levels.dedup();

    for i in 1..=input {
        if !all_levels.contains(&i) {
            println!("Oh, my keyboard!");
            return;
        }
    }
    println!("I become the guy.");
}
