use proconio::input;

fn main() {
    input! {
        a: i32, b: i32,
    }
    if a == b {
        println!("1");
        return;
    }
    println!("{}", 2 + if (a-b).abs()%2 == 0 { 1 } else { 0 });
}
