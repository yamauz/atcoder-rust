use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let ans = s
        .chars()
        .collect::<Vec<char>>()
        .chunks(2)
        .flat_map(|x| vec![x[1], x[0]])
        .collect::<String>();

    println!("{}", ans);
}
