use proconio::input;

fn main() {
    input! {
        n: usize
    }

    let ans = if n % 5 < 3 {
        5 * (n / 5)
    } else {
        5 * (n / 5) + 5
    };

    println!("{}", ans);
}
