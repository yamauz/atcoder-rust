use proconio::input;

fn main() {
    input! {
        x1:isize,
        y1:isize,
        x2:isize,
        y2:isize,
    }

    let dx = x2 - x1;
    let dy = y2 - y1;

    println!("{} {} {} {}", x2 - dy, y2 + dx, x1 - dy, y1 + dx);
}
