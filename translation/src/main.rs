fn main() {
    let mut s = String::new();
    let mut t = String::new();
    std::io::stdin()
        .read_line(&mut s)
        .expect("Failed to read input");
    std::io::stdin()
        .read_line(&mut t)
        .expect("Failed to read input"); //s.chars().rev().collect::<Vec<_>>() == t.chars().collect::<Vec<_>>()
    if s.trim().chars().rev().collect::<Vec<_>>() == t.trim().chars().collect::<Vec<_>>() {
        println!("YES");
    } else {
        println!("NO");
    }
}
