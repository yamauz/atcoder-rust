use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        s: Chars,
    }

    for (&x, &y) in s.iter().tuple_windows() {
        if (x == 'a' && y == 'b') || (x == 'b' && y == 'a') {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
