fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    std::io::stdin().read_line(&mut input1).unwrap();
    std::io::stdin().read_line(&mut input2).unwrap();
    let input1 = input1.trim();
    let input2 = input2.trim();
    let result: String = input1
        .chars()
        .zip(input2.chars())
        .map(|(x, y)| if x == y { '0' } else { '1' })
        .collect();

    println!("{result}");
}
