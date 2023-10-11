use proconio::input;

fn main() {
    input! {
        n: usize,
        mut t: [(String, usize); n],
    }

    let i = t.iter().enumerate().min_by_key(|(_, val)| val.1).unwrap().0;
    t.rotate_left(i);
    t.iter().for_each(|x| println!("{}", x.0));
}
