use proconio::input;
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        e: usize,
    }

    let mut arr = [0; 16];

    for i in [a, b, c, d, e] {
        arr[i - 1] += 1;
    }
    if arr.iter().any(|x| *x == 3) && arr.iter().any(|x| *x == 2) {
        println!("Yes");
    } else {
        println!("No");
    }
}
