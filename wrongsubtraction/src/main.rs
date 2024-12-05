fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("err");
    let mut values = input.split_whitespace();
    let mut n: i32 = values.next().unwrap().parse().unwrap();
    let k: i32 = values.next().unwrap().parse().unwrap();

    for _ in 0..k {
        if n % 10 == 0 {
            n /= 10;
        } else {
            n -= 1;
        }
    }
    println!("{n}");
}
