use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String;n]
    }

    for ss in s.iter().rev() {
        println!("{}", ss);
    }
}
