use proconio::input;

fn main() {
    input! {
        n: usize, k: usize,
        mut p: [i32; n],  // Vec<(i32, i32, i32)>
    }
    p.sort();
    let mut ans = 0;
    for i in &p[..k] {
        ans += i;
    }
    println!("{}", ans);
}
