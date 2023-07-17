use proconio::input;
use std::cmp::{max, min};

fn main() {
    input! {
        l1:i8,
        r1:i8,
        l2:i8,
        r2:i8
    }

    println!("{}", max(min(r1, r2) - max(l1, l2), 0));
}
