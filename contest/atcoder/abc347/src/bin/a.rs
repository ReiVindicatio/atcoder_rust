use proconio::input;

fn main() {
    input! {
        n: usize, k: i32,
        a: [i32; n]
    }
    for &i in &a {
        if i % k == 0 {
            print!("{} ", i/k);
        }
    }
    println!();
}
