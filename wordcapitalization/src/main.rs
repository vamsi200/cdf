fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("err");

    let inp: Vec<String> = input
        .trim()
        .split(' ')
        .map(|x| x.parse().expect("err"))
        .collect();

    for c in inp.iter() {
        if let Some(i) = c.chars().next() {
            print!("{}", i.to_uppercase());
        }
        for s in c.chars().skip(1) {
            print!("{s}");
        }
    }
}
