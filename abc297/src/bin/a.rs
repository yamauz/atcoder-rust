use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        t: [usize;n]
    }

    for i in t.windows(2) {
        if i[1] - i[0] <= d {
            println!("{}", i[1]);
            return;
        }
    }

    println!("-1");
}
