use proconio::input;
fn main() {
    input! {
        y:usize
    }

    if y % 4 == 2 {
        println!("{}", y);
    } else if y % 4 == 0 {
        println!("{}", y + 2);
    } else if y % 4 == 1 {
        println!("{}", y + 1);
    } else if y % 4 == 3 {
        println!("{}", y + 3);
    }
}
