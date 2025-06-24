use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        d: usize,
        s: Chars,
    }
    let count = s.iter().filter(|&&c| c == '.').count();
    println!("{}", count + d);
}
