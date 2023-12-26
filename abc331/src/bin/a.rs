use proconio::input;

fn main() {
    input! {
        month: usize,
        day: usize,
        mut y: usize,
        mut m: usize,
        mut d:usize
    }

    if day == d {
        d = 1;
        m += 1;
    } else {
        d += 1;
        println!("{} {} {}", y, m, d);
        return;
    }

    if m > month {
        m = 1;
        y += 1;
    }
    println!("{} {} {}", y, m, d);
}
