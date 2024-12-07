fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("err");
    let in_parsed: i32 = input.trim().parse().expect("err");
    let mut count = 0;
    for _ in 0..in_parsed {
        let mut n2 = String::new();
        std::io::stdin().read_line(&mut n2).expect("err");
        let n2_parsed: Vec<i32> = n2
            .split_whitespace()
            .map(|x| x.parse().expect("err"))
            .collect();
        let capacity = n2_parsed[1] - n2_parsed[0];
        if capacity >= 2 {
            count += 1;
        }
    }
    print!("{count}");
}
