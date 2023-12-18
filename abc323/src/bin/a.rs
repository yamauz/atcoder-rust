use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let pos = s
        .iter()
        .enumerate()
        .position(|(i, &v)| (i + 1) % 2 == 0 && v == '1');

    if let Some(i) = pos {
        if s[i] == '1' {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
