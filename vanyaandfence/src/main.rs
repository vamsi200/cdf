fn main() {
    let mut n1 = String::new();
    let mut n2 = String::new();
    std::io::stdin().read_line(&mut n1).expect("err");
    std::io::stdin().read_line(&mut n2).expect("err");

    let n1_parsed: Vec<i32> = n1
        .split_whitespace()
        .map(|x| x.parse().expect("err"))
        .collect();

    let n2_parsed: Vec<i32> = n2
        .split_whitespace()
        .map(|x| x.parse().expect("err"))
        .collect();
    let height = n1_parsed[1];
    let mut width = 0;

    for &n in n2_parsed.iter() {
        if height >= n {
            width += 1;
        } else {
            width += 2;
        }
    }
    println!("{:?}", width);
}
