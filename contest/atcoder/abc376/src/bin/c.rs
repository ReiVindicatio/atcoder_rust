use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        mut b: [usize; n-1],
    }
    a.sort();
    b.sort();
    for i in 0..n-1 {
        if a[i] > b[i] {
            println!("-1");
            return;
        }
    }
    let mut ng = 0;
    let mut ok = 1_000_000_001;
    while ok - ng > 1 {
        let mid = (ok + ng) / 2;
        let mut v = b.to_vec();
        v.push(mid);
        v.sort();
        let mut f = true;
        for i in 0..n {
            if a[i] > v[i] {
                f = false;
                break;
            }
        }
        if f {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    println!("{}", ok);
}
