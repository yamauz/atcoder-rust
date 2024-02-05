use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        s:String
    }

    let arr = s.split('.').collect_vec();

    println!("{}", arr.last().unwrap());
}
