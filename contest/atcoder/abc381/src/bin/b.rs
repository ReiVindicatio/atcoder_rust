use proconio::{input, marker::Chars};
use std::collections::HashSet;

fn main() {
    input! {
        s: Chars,
    }
    let n = s.len();
    if n % 2 == 1 {
        println!("No");
        return;
    }
    let mut set: HashSet<char> = HashSet::new();
    for i in 0..n {
        if i % 2 == 1 {
            continue;
        }
        if s[i] != s[i+1] {
            println!("No");
            return;
        }
        if set.contains(&s[i]) {
            println!("No");
            return;
        }
        set.insert(s[i]);
    }
    println!("Yes");
}
