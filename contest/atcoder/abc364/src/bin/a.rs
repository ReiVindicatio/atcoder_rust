use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String;n],
    }
    for i in 0..n - 1 {
        if s[i] == "sweet" && s[i+1] == "sweet" && i != n - 2{
            println!("No");
            return;
        }
    }
    println!("Yes");
}
