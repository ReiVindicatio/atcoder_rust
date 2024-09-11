use proconio::input;

fn main() {
    input! {
        n: u32,
    }
    match n {
        0..=125 => println!("4"),
        125..=211 => println!("6"),
        _ => println!("8"),
    }
}
