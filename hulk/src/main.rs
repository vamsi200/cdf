fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input_parsed: i32 = input.trim().parse().unwrap();

    let mut result = String::new();
    let mut s = "I hate it".to_string();

    if input_parsed == 1 {
        print!("{s}");
    }
    for i in 1..input_parsed {
        if i % 2 == 0 {
            result = s.replace("it", "that I hate it");
            s = result.clone();
        } else if i % 2 != 0 {
            result = s.replace("it", "that I love it");
            s = result.clone();
        }
    }
    println!("{result}");
}
