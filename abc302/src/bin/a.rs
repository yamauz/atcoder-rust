use proconio::input;

fn main() {
    input! {
        a:isize,
        b:isize,
    }

    if a % b == 0 {
        println!("{}", a / b);
    } else {
        println!("{}", a / b + 1);
    }
}
