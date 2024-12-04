use std::io;

fn main() {
    let mut num = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("err: couldnt read input");
    let n: usize = num.trim().parse().expect("err, not num");
    let mut problems = 0;
    // let total = problems + problems;

    for _ in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("err");
        let parsed_input: Vec<i32> = input
            .split_whitespace()
            .map(|x| x.parse().expect("err: invalid number"))
            .collect();
        let mut count = 0;
        //println!("{:?}", parsed_input);
        for i in parsed_input {
            if i == 1 {
                count += 1;
            }
        }
        // println!("{count}");
        if count >= 2 {
            problems += 1;
        }
    }
    println!("{problems}");
}
