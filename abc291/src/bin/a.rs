use proconio::input;

fn main() {
    input! {
        s: String,
    }

    s.chars().enumerate().for_each(|(i, v)| {
        if v.is_uppercase() {
            println!("{}", i + 1);
        }
    });
}
