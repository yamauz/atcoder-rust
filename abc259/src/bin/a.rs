use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        x:usize,
        t:usize,
        d:usize,
    }

    if m >= x {
        println!("{}", t);
    } else {
        println!("{}", (t - d * x) + d * m);
    }
}
