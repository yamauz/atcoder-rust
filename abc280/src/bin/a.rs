use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut cnt = 0;

    s.iter().for_each(|x| {
        cnt += x
            .iter()
            .fold(0, |sum, &x| if x == '#' { sum + 1 } else { sum })
    });

    println!("{}", cnt);
}
