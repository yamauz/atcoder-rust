use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        p: usize,
        q: usize,
        d: [usize;n],
    }

    let ans = min(p, q + d.iter().min().unwrap());
    println!("{}", ans);
}
