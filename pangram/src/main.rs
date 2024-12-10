fn main() {
    let mut input = String::new();
    let mut input_str = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let _input: i64 = input.trim().parse().unwrap();

    std::io::stdin().read_line(&mut input_str).unwrap();

    let input_chars = input_str.trim().to_lowercase();
    let mut okay = true;
    for alpha in 'a'..='z' {
        if !input_chars.contains(alpha) {
            println!("NO");
            okay = false;
            break;
        }
    }
    if okay {
        println!("YES");
    }
}
