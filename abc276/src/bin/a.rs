use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        a: Chars,
    }

    let mut arr = vec![];

    for (i, s) in a.iter().enumerate() {
        if *s == 'a' {
            arr.push(i);
        }
    }

    if (arr.len() == 0) {
        println!("{}", -1);
    } else {
        println!("{:?}", arr[arr.len() - 1] + 1);
    }
}
