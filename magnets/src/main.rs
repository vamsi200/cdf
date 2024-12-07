fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("err");
    let in_parsed: usize = input.trim().parse().expect("err");

    let mut count = 0;
    let mut p_magnet = String::new();
    for _ in 0..in_parsed {
        let c_magnet = std::io::stdin().lines().next().unwrap().unwrap();
        if c_magnet != p_magnet {
            count += 1;
        }
        p_magnet = c_magnet;
    }

    print!("{}", count);
}
