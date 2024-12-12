use std::collections::HashSet;

fn main() {
    let mut k = String::new();
    let mut l = String::new();
    let mut m = String::new();
    let mut n = String::new();
    let mut d = String::new();

    std::io::stdin().read_line(&mut k).unwrap();
    std::io::stdin().read_line(&mut l).unwrap();
    std::io::stdin().read_line(&mut m).unwrap();
    std::io::stdin().read_line(&mut n).unwrap();
    std::io::stdin().read_line(&mut d).unwrap();

    let k: i32 = k.trim().parse().unwrap();
    let l: i32 = l.trim().parse().unwrap();
    let m: i32 = m.trim().parse().unwrap();
    let n: i32 = n.trim().parse().unwrap();
    let d: i32 = d.trim().parse().unwrap();

    let mut damaged_dragons = HashSet::new();

    for i in (k..=d).step_by(k as usize) {
        damaged_dragons.insert(i);
    }
    for i in (l..=d).step_by(l as usize) {
        damaged_dragons.insert(i);
    }
    for i in (m..=d).step_by(m as usize) {
        damaged_dragons.insert(i);
    }
    for i in (n..=d).step_by(n as usize) {
        damaged_dragons.insert(i);
    }

    println!("{:?}", damaged_dragons.len());
}
