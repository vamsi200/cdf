fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("err");

    let mut is_upper_count = 0;
    let mut is_lower_count = 0;
    let input_chars = input.chars();
    for i in input_chars {
        if i.is_ascii_uppercase() {
            is_upper_count += 1;
        }
        if i.is_ascii_lowercase() {
            is_lower_count += 1;
        }
    }

    if is_upper_count > is_lower_count {
        print!("{}", input.to_uppercase());
    } else {
        print!("{}", input.to_lowercase());
    }
}
