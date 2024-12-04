fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("err");
    let mut x: Vec<String> = input
        .trim()
        .split('+')
        .map(|x| x.parse().expect("err"))
        .collect();

    x.sort();
    let output = x.join("+");
    println!("{output}");
}
