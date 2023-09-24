use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let arr = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"];
    let pos = arr.iter().position(|&p| p == s).unwrap();

    println!("{}", 5 - pos);
}
