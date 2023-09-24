use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        e: usize,
    }

    let arr = [a, b, c, d, e];

    println!("{}", arr.iter().unique().collect::<Vec<_>>().len());
}
