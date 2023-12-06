use proconio::{input, marker::Chars};

fn main() {
    input! {
        _:usize,
        s:Chars
    }

    let mut a = 0;
    let mut b = 0;
    let mut c = 0;

    for (i, &v) in s.iter().enumerate() {
        if v == 'A' {
            a += 1;
        }
        if v == 'B' {
            b += 1;
        }
        if v == 'C' {
            c += 1;
        }

        if a > 0 && b > 0 && c > 0 {
            println!("{}", i + 1);
            return;
        }
    }
}
