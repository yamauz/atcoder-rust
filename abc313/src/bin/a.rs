use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }

    let &m = p.iter().max().unwrap();

    if p.len() == 1 {
        println!("0");
        return;
    }
    let has_eq = p[1..].iter().any(|&x| x == p[0]);

    if p.len() == 1 || (p[0] == m && !has_eq) {
        println!("0");
    } else {
        println!("{}", (m + 1) - p[0]);
    }
}
