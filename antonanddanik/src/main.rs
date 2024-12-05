use std::io::{self};

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let _n: usize = input.trim().parse().expect("Failed to parse n");

    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let w: Vec<char> = input.trim().chars().collect();

    let a_count = w.iter().filter(|&&ch| ch == 'A').count();
    let d_count = w.iter().filter(|&&ch| ch == 'D').count();

    match a_count.cmp(&d_count) {
        std::cmp::Ordering::Greater => println!("Anton"),
        std::cmp::Ordering::Equal => println!("Friendship"),
        std::cmp::Ordering::Less => println!("Danik"),
    }
}
