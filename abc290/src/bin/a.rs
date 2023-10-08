use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m],
    }

    let ans = a.iter().enumerate().fold(
        0,
        |sum, (i, v)| if b.contains(&(i + 1)) { sum + v } else { sum },
    );

    println!("{}", ans);
}
