use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let mut count = 0;
    for i in 1..s.len() {
        if s[i] == '-' {
            count += 1;
        } else {
            print!("{} ", count);
            count = 0;
        }
    }
    println!();
}
