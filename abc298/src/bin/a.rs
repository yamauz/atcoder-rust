use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        s: Chars
    }

    if s.iter().filter(|&&c| c == 'x').count() >= 1 {
        println!("No");
    } else if s.iter().filter(|&&c| c == 'o').count() >= 1 {
        println!("Yes");
    } else {
        println!("No");
    }
}
