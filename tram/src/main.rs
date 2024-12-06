fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).expect("err");
    let n_parsed: i32 = n.trim().parse().expect("err");

    let mut current_p = 0;
    let mut capacity = 0;

    for _ in 0..n_parsed {
        let mut p = String::new();
        std::io::stdin().read_line(&mut p).expect("err");
        let out: Vec<i32> = p
            .split_whitespace()
            .map(|x| x.parse().expect("err"))
            .collect::<Vec<_>>();
        let a = out[0];
        let b = out[1];

        current_p -= a;
        current_p += b;

        capacity = capacity.max(current_p);
    }
    println!("{capacity}");
}
