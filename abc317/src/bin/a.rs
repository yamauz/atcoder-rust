use proconio::input;

fn main() {
    input! {
        n: usize,
        h: usize,
        x: usize,
        p: [usize; n],
    }

    for (i, v) in p.iter().enumerate() {
        if x <= h + v {
            println!("{}", i + 1);
            return;
        }
    }
}
