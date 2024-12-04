use std::collections::HashSet;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("err");

    let p_input: Vec<String> = input
        .trim()
        .split(' ')
        .map(|x| x.parse().expect("err"))
        .collect();

    for i in p_input.iter() {
        let chars: Vec<_> = i.chars().collect();
        let mut h = HashSet::new();
        let unique: Vec<_> = chars.into_iter().filter(|x| h.insert(*x)).collect();

        if unique.len() % 2 == 0 {
            println!("CHAT WITH HER!")
        } else {
            println!("IGNORE HIM!");
        }
    }
}
