use std::io;

fn main() {
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Error");
    match num.trim().parse::<i32>() {
        Ok(input) => {
            // Special case for 2
            if input == 2 {
                println!("NO");
            } else if input % 2 == 0 {
                println!("YES");
            } else {
                println!("NO");
            }
        }
        Err(err) => eprintln!("{err}"),
    }
}
