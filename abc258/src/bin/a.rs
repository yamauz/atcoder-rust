use proconio::input;

fn main() {
    input! {
        k: usize,
    }

    let h = k / 60;
    let m = k % 60;

    println!("{}:{:0>2}", h + 21, m);
}
