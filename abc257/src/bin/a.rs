use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
    }

    let ans = (b'A' + ((x - 1) / n) as u8) as char;
    println!("{}", ans);
}
