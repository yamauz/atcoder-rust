use proconio::input;

fn main() {
    input! {
        s:String,
    }
    let ans = match &s[..] {
        "ACE" => "Yes",
        "BDF" => "Yes",
        "CEG" => "Yes",
        "DFA" => "Yes",
        "EGB" => "Yes",
        "FAC" => "Yes",
        "GBD" => "Yes",
        _ => "No",
    };

    println!("{}", ans);
}
