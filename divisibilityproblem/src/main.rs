fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).unwrap();
    let n_parsed: i32 = n.trim().parse().unwrap();

    for _ in 0..n_parsed {
        let mut values = String::new();
        std::io::stdin().read_line(&mut values).unwrap();
        let v: Vec<i32> = values
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let a = v[0];
        let b = v[1];
        let remainder = a % b;

        let count = if remainder == 0 { 0 } else { b - remainder };
        println!("{count}");
    }
}
