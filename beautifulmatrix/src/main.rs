use std::io;

fn main() {
    let mut matrix = Vec::new();
    let x = 5;
    for _ in 0..x {
        let mut m = String::new();
        io::stdin().read_line(&mut m).expect("err");
        let row: Vec<i32> = m
            .split_whitespace()
            .map(|x| x.parse().expect("err"))
            .collect();
        matrix.push(row);
    }

    let mut position = (0, 0);

    for (i, row) in matrix.iter().enumerate() {
        for (j, &value) in row.iter().enumerate() {
            if value == 1 {
                position = (i, j);
                break;
            }
        }
    }
    let moves = (position.0 as i32 - 2).abs() + (position.1 as i32 - 2).abs();
    println!("{}", moves);
}
