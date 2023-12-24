use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        s: [usize; n],
    }

    let ans = s.iter().filter(|&&a| a <= x).sum::<usize>();

    println!("{}", ans);
}
