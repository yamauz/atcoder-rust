use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let ss = s
        .chars()
        .map(|x| if x == '0' { '1' } else { '0' })
        .collect::<String>();

    println!("{}", ss);
}
