use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut a = vec![vec![]; n];
    for i in 0..n {
        for _ in 0..=i {
            input! {
                val: usize,
            }
            a[i].push(val);
        }
    }
    let mut now: usize = 0;
    for i in 0..n {
        let left = (i).max(now);
        let right = (i).min(now);
        // dbg!("{} {}", left, right);
        now = a[left][right] - 1;
    }
    println!("{}", now+1);
}
