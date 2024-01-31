use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        x: isize,
        y: isize,
        xx: [isize; n],
        yy: [isize; m],
    }

    let xx_max = xx.into_iter().max().unwrap();
    let yy_min = yy.into_iter().min().unwrap();

    let z = x.max(xx_max) + 1;

    if yy_min >= z && z <= y {
        println!("No War");
    } else {
        println!("War");
    }
}
