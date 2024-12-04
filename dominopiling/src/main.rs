use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("err");

    let domino: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().expect("err"))
        .collect();

    let m = domino[0] as usize;
    let n = domino[1] as usize;
    let mut total_squares = m * n;
    if total_squares % 2 != 0 {
        total_squares -= 1;
    }
    let max_no_of_dominos = total_squares / 2;
    println!("{max_no_of_dominos}");
}
