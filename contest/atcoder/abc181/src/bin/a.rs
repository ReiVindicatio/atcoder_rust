use proconio::input;

fn main() {
    input! {
        n: i32
    }
    println!("{}", if n & 1 == 1 { "Black" } else { "White" });
}
