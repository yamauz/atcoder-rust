use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let start = if n > k { a[k..].to_vec() } else { vec![] };
    let zero = vec![0; k];

    let mut result = Vec::new();

    result.extend(start);
    result.extend(zero);

    let res = if result.len() > n {
        &result[..n]
    } else {
        &result
    };

    res.iter().for_each(|x| println!("{}", x));
}
