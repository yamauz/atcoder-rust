use proconio::input;

fn main() {
    input! {
        n: usize,
        w: [String; n],
    }

    if w.iter()
        .any(|ss| ss == "and" || ss == "not" || ss == "that" || ss == "the" || ss == "you")
    {
        println!("Yes");
    } else {
        println!("No");
    };
}
