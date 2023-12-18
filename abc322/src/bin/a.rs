use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        n: Chars,
    }

    for (i, v) in n.windows(3).enumerate() {
        if v[0] == 'A' && v[1] == 'B' && v[2] == 'C' {
            println!("{}", i + 1);
            return;
        }
    }

    println!("{}", -1);
}
