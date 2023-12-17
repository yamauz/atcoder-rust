use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut list = HashMap::new();
    list.insert(String::from("tourist"), 3858);
    list.insert(String::from("ksun48"), 3679);
    list.insert(String::from("Benq"), 3658);
    list.insert(String::from("Um_nik"), 3648);
    list.insert(String::from("apiad"), 3638);
    list.insert(String::from("Stonefeang"), 3630);
    list.insert(String::from("ecnerwala"), 3613);
    list.insert(String::from("mnbvmar"), 3555);
    list.insert(String::from("newbiedmy"), 3516);
    list.insert(String::from("semiexp"), 3481);

    let ans = list.get(&s).unwrap();

    println!("{}", ans);
}
