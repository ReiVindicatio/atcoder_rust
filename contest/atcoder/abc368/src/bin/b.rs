use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i32; n],
    }
    for i in 0..10000 {
        let mut cnt = 0;
        for &i in &a {
            cnt += if i > 0 { 1 } else { 0 };
        }
        if cnt <= 1 {
            println!("{}", i);
            return;
        }
        a.sort_by(|a, b| b.cmp(&a));
        a[0] = a[0] - 1;
        a[1] = a[1] - 1;
    }
}
