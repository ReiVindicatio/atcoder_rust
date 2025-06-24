use proconio::input;

fn main() {
    input! {
        a: i32, b: i32,
    }
    if a == b {
        println!("1");
    } else if (a.max(b) - a.min(b))%2 == 0 {
        println!("3");
    } else {
        println!("2");
    }
}
