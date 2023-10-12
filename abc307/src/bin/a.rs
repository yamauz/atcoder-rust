use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n * 7],
    }

    for i in a.chunks(7) {
        print!("{} ", i.iter().sum::<usize>());
    }
}
