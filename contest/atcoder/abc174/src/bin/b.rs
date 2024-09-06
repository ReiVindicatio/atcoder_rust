use proconio::input;

fn main() {
    input! {
        n: usize,
        d: i64,
        xy: [(i64, i64); n],
    }
    let mut ans = 0;
    for &(x, y) in &xy {
        if x*x + y*y <= d*d {
            ans += 1;
        }
    }
    println!("{}", ans);
}
