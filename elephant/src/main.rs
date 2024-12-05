fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("err");
    let parsed_input: i32 = input.trim().parse().expect("err");

    let moves = parsed_input / 5;
    let rem = parsed_input % 5;

    if rem > 0 {
        println!("{}", moves + 1);
    } else {
        println!("{}", moves);
    }
}
