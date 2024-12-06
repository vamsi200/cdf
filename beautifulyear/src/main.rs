fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("err");
    let mut input_parsed: i32 = input.trim().parse().expect("err");

    loop {
        input_parsed += 1;
        let input_str = input_parsed.to_string();
        let mut digits = input_str.chars().collect::<Vec<_>>();
        digits.sort_unstable();
        digits.dedup();

        if digits.len() == input_str.len() {
            println!("{}", input_parsed);
            break;
        }
    }
}
