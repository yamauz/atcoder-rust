use proconio::input;

fn main() {
    input! {
        n: usize,
        t: [String; n],
    }

    let a = t.iter().filter(|&t| t == "For").count();
    let b = t.iter().filter(|&t| t == "Against").count();

    println!("{}", if a > b { "Yes" } else { "No" });
}
