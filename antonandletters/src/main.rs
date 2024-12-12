use std::collections::HashSet;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let parsed: String = input.trim().parse().unwrap();
    let b = parsed
        .trim_start_matches('{')
        .trim_end_matches('}')
        .split(", ")
        .filter(|x| !x.is_empty());

    let mut hashset = HashSet::new();

    for i in b {
        hashset.insert(i);
    }

    println!("{}", hashset.len());
}
