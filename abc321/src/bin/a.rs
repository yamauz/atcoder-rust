use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars
    }

    if n.iter()
        .tuple_windows()
        .all(|(a, b)| a.to_string().parse::<i32>().unwrap() > b.to_string().parse::<i32>().unwrap())
    {
        println!("Yes");
    } else {
        println!("No");
    }
}
