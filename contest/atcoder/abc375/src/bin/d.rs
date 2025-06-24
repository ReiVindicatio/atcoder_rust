use proconio::{input, marker::Chars};
use std::collections::HashMap;

fn main() {
    input! {
        s: Chars,
    }
    let mut l: HashMap<char, i64> = HashMap::new();
    let mut r: HashMap<char, i64> = HashMap::new();
    for &c in s.iter() {
        *r.entry(c).or_insert(0) += 1;
    }
    let mut ans = 0 as i64;
    for &c in s.iter() {
        *r.entry(c).or_insert(0) -= 1;
        for (k, v) in &l {
            if let Some(t) = r.get(k) {
                ans += t * v;
            } 
        }
        *l.entry(c).or_insert(0) += 1;
    }
    println!("{}", ans);
}
