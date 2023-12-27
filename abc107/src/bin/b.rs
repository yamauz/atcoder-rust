use proconio::input;

fn main() {
    input! {
        h:usize,
        w:usize,
        aw: [String;h],
    }

    let mut count: Vec<usize> = vec![];

    for row in aw.iter() {
        for (i, v) in row.chars().enumerate() {
            if v == '.' {
                count.push(i);
            }
        }
    }
    let mut col_count: Vec<usize> = vec![];

    for i in 0..w {
        if count.iter().filter(|&&c| c == i).count() == h {
            col_count.push(i);
        }
    }

    for row in aw.iter() {
        if row.chars().all(|s| s == '.') {
            continue;
        }

        let filtered: String = row
            .chars()
            .enumerate()
            .filter(|&(i, _)| !col_count.contains(&i))
            .map(|(_, c)| c)
            .collect();

        println!("{}", filtered);
    }
}
