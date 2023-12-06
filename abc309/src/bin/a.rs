use proconio::input;

fn main() {
    input! {
        a: isize,
        b: isize,
    }

    if a + 1 == b && a % 3 != 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
