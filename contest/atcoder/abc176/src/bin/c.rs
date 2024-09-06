use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut m = 0;
    let mut ans = 0;
    for &i in &a {
        if i > m {
            m = i;
        }
        ans += m - i;
    }
    println!("{}", ans);
}
