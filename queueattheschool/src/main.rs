fn main() {
    let mut n1 = String::new();
    let mut n2 = String::new();

    std::io::stdin().read_line(&mut n1).expect("err");
    std::io::stdin().read_line(&mut n2).expect("err");

    let n1_parsed: Vec<i32> = n1
        .split_whitespace()
        .map(|x| x.parse().expect("err"))
        .collect();

    let _n = n1_parsed[0];
    let t = n1_parsed[1] as usize;
    //let mut out: Vec<char> = Vec::new();

    let mut n2_parsed: Vec<char> = n2.trim().chars().collect();
    for _ in 0..t {
        let mut i = 0;
        while i < n2_parsed.len() - 1 {
            if n2_parsed[i] == 'B' && n2_parsed[i + 1] == 'G' {
                n2_parsed.swap(i, i + 1);
                i += 1;
            }
            i += 1;
        }
    }

    let result: String = n2_parsed.into_iter().collect();
    println!("{result}");
}
