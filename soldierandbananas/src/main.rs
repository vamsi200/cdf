fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("err");
    let parsed_input: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().expect("err"))
        .collect();

    let k = parsed_input[0];
    let n = parsed_input[1];
    let w = parsed_input[2];
    let mut cost = 0;

    for _ in 0..(n + 1) {
        cost = k * (w * (w + 1)) / 2;
    }
    if cost > n {
        println!("{}", cost - n);
    } else {
        println!("0");
    }
}
