use proconio::input;

fn main() {
    input! {
        n: i64, y: i64,
    }
    for a in 0..=n {
        for b in 0..=n {
            if a*10000 + b*5000 <= y && (y - a*10000 - b*5000)/1000 + a + b == n {
                println!("{} {} {}", a, b, n-a-b);
                return;
            }
        }
    }
    println!("-1 -1 -1");
}
