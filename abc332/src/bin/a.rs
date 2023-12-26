use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        k: usize,
        pq: [(usize,usize); n],
    }

    let sum = pq.iter().map(|(p, q)| p * q).sum::<usize>();

    if sum >= s {
        println!("{}", sum);
    } else {
        println!("{}", sum + k);
    }
}
