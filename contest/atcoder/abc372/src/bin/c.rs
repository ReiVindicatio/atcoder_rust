use proconio::{input, marker::Chars, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        mut s: Chars,
    }
    let mut ans = 0;
    for i in 0..n-2 {
        if s[i] == 'A' && s[i+1] == 'B' && s[i+2] == 'C' {
            ans += 1;
        }
    }
    for _ in 0..q {
        input! {
            x: Usize1,
            c: char,
        }
        for i in 0..3 {
            if x as i32 - (2 - i as i32) < 0 {
                continue;
            }
            let pos = x  - (2 - i);
            if pos + 2 >= n {
                continue;
            }
            if s[pos] == 'A' && s[pos+1] == 'B' && s[pos+2] == 'C' {
                ans -= 1;
            } 
        }
        s[x] = c;
        for i in 0..3 {
            if x as i32 - (2 - i as i32) < 0 {
                continue;
            }
            let pos = x  - (2 - i);
            if pos + 2 >= n {
                continue;
            }
            if s[pos] == 'A' && s[pos+1] == 'B' && s[pos+2] == 'C' {
                ans += 1;
            } 
        }
        println!("{}", ans);
    }
}
