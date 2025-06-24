use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    println!("{}", s.iter().filter(|&&x| x != '.').collect::<String>());
}
