use proconio::input;

fn main() {
    input! {
        n: u8,
    }

    for i in 0..n {
        print!("{}", (b'A' + i) as char);
    }
}
