use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }

    for (i, v) in s.iter().enumerate() {
        if i + 1 == s.len() {
            println!("{}", 4);
        } else {
            print!("{}", v);
        }
    }
}
