use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    println!("{}", f(n));
}

fn f(k: usize) -> usize {
    if k == 0 {
        return 1_usize;
    }
    k * f(k - 1)
}
