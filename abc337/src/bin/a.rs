use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(usize,usize);n]
    }

    let total = xy.iter().fold((0, 0), |acc, (x, y)| (acc.0 + x, acc.1 + y));

    if total.0 > total.1 {
        println!("Takahashi");
    } else if total.0 < total.1 {
        println!("Aoki");
    } else {
        println!("Draw");
    }
}
