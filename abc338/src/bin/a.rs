use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars,
    }

    if n.iter()
        .enumerate()
        .all(|(i, v)| (i == 0 && v.is_uppercase()) || i != 0 && v.is_lowercase())
    {
        println!("Yes");
    } else {
        println!("No");
    }
}
