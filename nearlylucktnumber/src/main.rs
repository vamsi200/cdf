fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("err");
    let parsed_input: i64 = input.trim().parse().expect("err");

    let luckynumbers: Vec<char> = vec!['4', '7'];

    let mut count = 0;
    let n: Vec<_> = parsed_input.to_string().chars().collect();
    for i in n.iter() {
        if luckynumbers.contains(i) {
            count += 1;
        }
    }

    if count == 4 || count == 7 {
        println!("YES");
    } else {
        println!("NO");
    }
}
