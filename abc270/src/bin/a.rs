use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    let mut s: Vec<usize> = vec![0; 4];

    let t: Vec<usize> = match a {
        0 => vec![],
        1 => vec![1],
        2 => vec![2],
        3 => vec![1, 2],
        4 => vec![4],
        5 => vec![4, 1],
        6 => vec![4, 2],
        _ => vec![4, 2, 1],
    };
    let a: Vec<usize> = match b {
        0 => vec![],
        1 => vec![1],
        2 => vec![2],
        3 => vec![1, 2],
        4 => vec![4],
        5 => vec![4, 1],
        6 => vec![4, 2],
        _ => vec![4, 2, 1],
    };

    for i in t {
        s[i - 1] += 1;
    }
    for i in a {
        s[i - 1] += 1;
    }

    let mut ans = 0;

    if s[0] != 0 {
        ans += 1;
    }
    if s[1] != 0 {
        ans += 2;
    }
    if s[3] != 0 {
        ans += 4;
    }
    println!("{}", ans);
}
