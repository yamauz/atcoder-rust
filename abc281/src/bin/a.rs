use proconio::input;

fn main() {
    input! {
        mut n:isize,
    }

    while n != -1 {
        println!("{}", n);
        n -= 1;
    }
}
