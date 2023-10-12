use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        s: [usize; 8],
    }

    println!(
        "{}",
        if s.iter()
            .tuple_windows()
            .all(|(x, y)| x <= y && (&100..=&675).contains(&x) && x % 25 == 0)
        {
            "Yes"
        } else {
            "No"
        }
    );
}
