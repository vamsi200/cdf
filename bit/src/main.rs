use std::io;

fn main() {
    let mut num = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("err: failed to read");
    let n: usize = num.trim().parse().expect("err: coulndt parse");
    let mut final_value = 0;
    for _ in 0..n {
        let mut operation = String::new();
        io::stdin().read_line(&mut operation).expect("err");

        if operation.contains("++") {
            final_value += 1;
        }
        if operation.contains("--") {
            final_value -= 1;
        }
    }
    println!("{final_value}");
}
