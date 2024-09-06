use proconio::input;

fn main() {
    input! {
        n: usize,
        mut l: [i32; n],
    }
    l.sort();
    let mut ans = 0;
    for i in 0..n {for j in i+1..n {for k in j+1..n {
        ans += if l[i] + l[j] > l[k] && l[i] != l[j] && l[j] != l[k] { 1 } else { 0 };
    }}}
    println!("{}", ans);
}
