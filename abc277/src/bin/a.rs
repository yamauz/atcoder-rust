use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        p: [usize; n],
    }

    let index = p.iter().enumerate().find(|(_, u)| **u == x).unwrap();

    println!("{:?}", index.0 + 1);
}
