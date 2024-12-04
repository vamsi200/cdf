use std::io;

fn main() {
    let mut n_str = String::new();
    io::stdin().read_line(&mut n_str).expect("err");
    let n: usize = n_str.trim().parse().expect("invalid");

    for _ in 0..n {
        let mut word = String::new();
        io::stdin().read_line(&mut word).expect("err");
        let word = word.trim();

        if word.len() > 10 {
            let out = format!(
                "{}{}{}",
                word.chars().next().unwrap(),
                word.len() - 2,
                word.chars().last().unwrap()
            );
            println!("{}", out);
        } else {
            println!("{}", word);
        }
    }
}
