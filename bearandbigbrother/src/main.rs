use std::io;

fn main() {
    let mut weight = String::new();
    io::stdin().read_line(&mut weight).expect("err");

    let two_weights: Vec<i32> = weight
        .split_whitespace()
        .map(|x| x.parse().expect("err"))
        .collect();
    let weight_increase_of_limak = 3;
    let weight_increase_of_bob = 2;

    let mut count = 0;

    let mut weight_of_limak = two_weights[0];
    let mut weight_of_bob = two_weights[1];

    while weight_of_limak <= weight_of_bob {
        weight_of_limak *= weight_increase_of_limak;
        weight_of_bob *= weight_increase_of_bob;
        count += 1;
    }
    println!("{count}");
}
