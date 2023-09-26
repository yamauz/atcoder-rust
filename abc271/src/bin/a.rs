use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let a = n / 16;
    let b = n % 16;

    println!("{}{}", convert(a), convert(b));
}

fn convert(n: usize) -> String {
    match n {
        10 => "A".to_string(),
        11 => "B".to_string(),
        12 => "C".to_string(),
        13 => "D".to_string(),
        14 => "E".to_string(),
        15 => "F".to_string(),
        _ => n.to_string(),
    }
}
