#[allow(unused)]
use proconio::{marker::*, *};

fn main() {
    input! {
        mut s: Chars,
        t: Chars,
    }
    let n = s.len();
    let mut x = vec![];
    while s != t {
        let mut tmp = vec![];
        for i in 0..n {
            if s[i] != t[i] {
                let mut u = s.clone();
                u[i] = t[i];
                tmp.push(u);
            }
        }
        tmp.sort();
        x.push(tmp[0].iter().collect::<String>());
        s = tmp[0].clone();
    }
    println!("{}", x.len());
    for s in &x {
        println!("{}", s);
    }
}
