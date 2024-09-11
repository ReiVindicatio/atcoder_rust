use proconio::input;

fn main() {
    input! {
        n: usize, k: usize,
        a: [i32; n],
    }
    for i in n-k..n {
        print!("{} ", a[i]);
    }
    for i in 0..n-k {
        print!("{} ", a[i]);
    }
    println!();
}
