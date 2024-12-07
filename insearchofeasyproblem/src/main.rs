fn main() {
    let mut n1 = String::new();
    let mut n2 = String::new();

    std::io::stdin().read_line(&mut n1).expect("err");
    std::io::stdin().read_line(&mut n2).expect("err");

    let n2_parsed: Vec<i32> = n2
        .split_whitespace()
        .map(|x| x.parse().expect("err"))
        .collect();
    let result = n2_parsed.into_iter().find(|&x| x == 1);
    match result {
        Some(_) => println!("HARD"),
        None => println!("EASY"),
    }
}
