use proconio::input;
fn main() {
    input! {
        s:String,
    }

    for c in s.chars() {
        let ans = s.chars().filter(|&x| x == c).count();

        if ans == 1 {
            println!("{}", c);
            return;
        }
    }

    println!("{}", -1);
}
