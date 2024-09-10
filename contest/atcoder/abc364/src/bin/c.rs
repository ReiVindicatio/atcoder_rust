use proconio::input;

fn main() {
    input! {
        n: usize, x: i64, y: i64,
        a: [i64; n], b: [i64; n],
    }
    let mut ab: Vec<(_, _)> = a.into_iter().zip(b.into_iter()).collect();
    let mut ans = n+10;
    ab.sort_by(|a, b| a.0.cmp(&b.0).reverse());
    let mut sum: i64 = 0;
    let mut count = 0;
    for &(a, _) in &ab {
        sum += a;
        count += 1;
        if sum > x {
            break;
        }
    }
    ans = ans.min(count);
    count = 0;
    sum = 0;
    ab.sort_by(|a, b| a.1.cmp(&b.1).reverse());
    for &(_, b) in &ab {
        sum += b;
        count += 1;
        if sum > y {
            break;
        }
    }
    println!("{}", ans.min(count));
}
