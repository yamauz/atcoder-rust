use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        p: usize,
    }

    let mut sum = m;
    let mut cnt = 1;

    loop {
        if n == sum {
            println!("{}", cnt);
            return;
        }
        if n > sum {
            return;
        }
        sum = sum + p;

        cnt += 1;
    }
}
