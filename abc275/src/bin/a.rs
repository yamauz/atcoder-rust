use proconio::input;

fn main() {
    input! {
        n: usize,
        mut  h: [usize; n],
    }

    if h.len() == 1 {
        println!("{}", 1);
    } else {
        let max_index = h.iter().enumerate().max_by_key(|&(_, x)| x).unwrap().0;
        println!("{}", max_index + 1);
    }
}
