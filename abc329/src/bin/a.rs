use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let ans = s.iter().join(" ");

    println!("{}", ans);
}
