use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize;n]
    }

    if a.iter().tuple_windows().all(|(x, y)| x == y) {
        println!("Yes");
    } else {
        println!("No");
    }
}
