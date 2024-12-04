use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("err");

    let parsed_input: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().expect("err: not a number"))
        .collect();

    let _n = parsed_input[0] as usize;
    let k = parsed_input[1] as usize;
    let mut advanced_participants = 0;
    let mut score = String::new();

    io::stdin().read_line(&mut score).expect("err");
    let parsed_score: Vec<i32> = score
        .split_whitespace()
        .map(|x| x.parse().expect("err: not a number"))
        .collect();
    let score_at_k = parsed_score[k - 1];

    for i in parsed_score.iter() {
        if parsed_score[0] == 0 {
            advanced_participants = 0;
            break;
        }
        if i >= &score_at_k {
            advanced_participants += 1;
        }
        if score_at_k == 0 && *i == 0 {
            advanced_participants -= 1;
        }
    }
    println!("{advanced_participants}");
}
