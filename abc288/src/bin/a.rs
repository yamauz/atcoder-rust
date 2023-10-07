use proconio::input;

fn main() {
    input! {
        n:usize,
        l:[[i32;2];n]
    }

    for i in l {
        println!("{}", i.iter().sum::<i32>());
    }
}
