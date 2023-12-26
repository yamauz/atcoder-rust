use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    for _ in 1..=n {
        print!("{}", n);
    }
}
