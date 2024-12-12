fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let mut parsed_input: usize = input.trim().parse().unwrap();
    let bills = [100, 20, 10, 5, 1];
    let mut count = 0;

    for &bill in bills.iter() {
        count += parsed_input / bill;
        parsed_input %= bill;
    }
    println!("{count}");
}
