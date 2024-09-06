use proconio::input;

fn main() {
    input! {
        k: i64, n: usize,
        a: [i64; n],
    }
    let mut ans = a[0] + k - a[n-1];
    for i in 0..n-1 {
        ans = std::cmp::max(ans, a[i+1] - a[i]);
    }
    println!("{}", k-ans);
}
