use proconio::input;

fn main() {
    input! {
        a: isize,
        b: isize,
    }

    if (a > b && a - b <= 3) || (a < b && b - a <= 2) {
        println!("Yes");
    } else {
        println!("No");
    }
}
