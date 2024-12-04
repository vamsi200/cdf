fn main() {
    let mut input = String::new();

    std::io::stdin().read_line(&mut input).expect("err");
    let n: usize = input.trim().parse().expect("err");

    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("err");
    let s = s.trim().to_string();

    let mut count = 0;

    for i in 0..(n - 1) {
        if s.chars().nth(i) == s.chars().nth(i + 1) {
            count += 1;
        }
    }

    println!("{}", count);
}
