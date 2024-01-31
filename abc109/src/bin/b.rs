use proconio::input;

fn main() {
    input! {
        n: usize,
        w: [String; n],
    }

    let mut list: Vec<String> = vec![];

    for (i, s) in w.iter().enumerate() {
        if list.contains(s) {
            println!("No");
            return;
        }

        if i == 0 || (s.chars().next().unwrap() == list.get(i - 1).unwrap().chars().last().unwrap())
        {
            list.push(s.to_string());
        } else {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
