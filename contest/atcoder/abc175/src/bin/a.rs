use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let mut count = 0;
    let mut ans = 0;
    for c in s.chars() {
        match c {
            'R' => count += 1,
            _ => {
                ans = ans.max(count);
                count = 0;
            },
        }
    }
    println!("{}", ans.max(count));
}
