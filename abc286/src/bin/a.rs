use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        p: usize,
        q: usize,
        r: usize,
        s: usize,
        mut a: [usize; n],
    }

    for (i, j) in (p - 1..q).zip(r - 1..s) {
        a.swap(i, j);
    }

    println!("{}", a.iter().join(" "));
}
