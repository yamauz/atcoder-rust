use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        s: Chars,
    }

    for i in s {
        print!("{}", i);
        print!("{}", i);
    }
}
