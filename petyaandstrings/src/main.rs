fn main() {
    let mut strings: Vec<String> = Vec::new();
    for _ in 0..2 {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).expect("err");
        strings.push(s.trim().to_lowercase().to_string());
    }

    if strings[0] == strings[1] {
        println!("0");
    }
    if strings[0] < strings[1] {
        println!("-1");
    }
    if strings[0] > strings[1] {
        println!("1");
    }
}
