use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars
    }

    let t = s.iter().filter(|&&c| c == 'T').count();
    let a = s.iter().filter(|&&c| c == 'A').count();

    if t == a {
        if s.iter().rposition(|&c| c == 'T').unwrap() > s.iter().rposition(|&c| c == 'A').unwrap() {
            println!("A");
        } else {
            println!("T");
        }
    } else if t < a {
        println!("A");
    } else {
        println!("T");
    }
}
