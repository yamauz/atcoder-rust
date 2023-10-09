use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: String,
    }

    if s.chars().tuple_windows().all(|(a, b)| {
        println!("{}{}", a, b);
        return a != b;
    }) {
        println!("Yes")
    } else {
        println!("No");
    };
}
