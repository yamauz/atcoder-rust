use proconio::input;

fn main() {
    input! {
        l: usize,
        r: usize,
    }

    "atcoder".chars().enumerate().for_each(|(i, x)| {
        if i >= l - 1 && i < r {
            print!("{}", x)
        }
    })
}
