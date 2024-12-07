fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input_parsed: u64 = input.trim().parse().unwrap();

    if input_parsed % 2 == 0 {
        println!("{}", input_parsed / 2);
    } else {
        println!("-{}", (input_parsed + 1) / 2);
    }
}
