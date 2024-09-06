use proconio::input;

fn main() {
    input! {
        n: usize,
        v: [(i64, i64); n],
    }
    let mut ans = 0;
    for &(a, b) in &v {
        ans += b*(b+1)/2 - a*(a-1)/2;
    }
    println!("{}", ans);
}
