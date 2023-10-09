use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let lb = s.iter().position(|&c| c == '|').unwrap();
    let rb = s.iter().rposition(|&c| c == '|').unwrap();
    let a = s.iter().position(|&c| c == '*').unwrap();

    if lb < a && a < rb {
        println!("in");
    } else {
        println!("out");
    }
}
