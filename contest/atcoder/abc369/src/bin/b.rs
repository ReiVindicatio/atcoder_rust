use proconio::input;

fn main() {
    input! {
        n: usize,
        A: [i32; n],
    }
    let mut l = -1;
    let mut r = -1;
    let mut ans = 0;
    for &(a, s) in &A {
        if s == 'L' {
            if l != -1 {
                ans += (l - a).abs();
            }
            l = a;
        } else {
            if r != -1 {
                ans += (r - a).abs();
            }
            r = a;
        }
    } 
    println!("{}", ans);
}
