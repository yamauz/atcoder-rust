use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }

    let ans = s
        .iter()
        .fold(0, |sum, &x| if x == 'v' { sum + 1 } else { sum + 2 });

    println!("{}", ans);
}
