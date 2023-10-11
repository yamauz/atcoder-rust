use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        s: Chars,
        t: Chars,
    }

    let ans = s.iter().zip(t).all(|(&a, b)| {
        a == b
            || ((a == 'l' || a == '1') && (b == 'l' || b == '1'))
            || ((a == '0' || a == 'o') && (b == '0' || b == 'o'))
    });

    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}
