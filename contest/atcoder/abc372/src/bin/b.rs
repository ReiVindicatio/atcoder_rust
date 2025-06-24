use proconio::input;

fn main() {
    input! {
        mut m: i32,
    }
    let mut ans = vec![];
    let mut count = 0;
    while m > 0 {
        for _ in 0..m%3 {
            ans.push(count);
        }
        count += 1;
        m /= 3;
    }
    println!("{}", ans.len());
    for i in ans.iter() {
        print!("{} ", i);
    }
    println!();
}
