use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
        n: usize,
    }

    let three = (n / 3) * y;
    let one = (n % 3) * x;

    if x * n > three + one {
        println!("{}", three + one);
    } else {
        println!("{}", x * n);
    }
}
