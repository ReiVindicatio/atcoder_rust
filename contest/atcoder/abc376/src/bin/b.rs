use proconio::input;

fn main() {
    input! {
        n: i32,
        q: usize,
    }
    let mut l = 0;
    let mut r = 1;
    let mut val = 0;
    for _ in 0..q {
        input! {
            h: char,
            mut t: i32,
        }
        t -= 1;
        if h == 'L' {
            let mut ll = l;
            let mut ans = 0;
            while ll != t {
                ll += 1;
                ans += 1;
                ll %= n;
                if ll == r {
                    break;
                }
            }
            if ll == t {
                val += ans;
                l = t;
                continue;
            }
            ans = 0;
            while l != t {
                l -= 1;
                l += n;
                l %= n;
                ans += 1;
            }
            val += ans;
        } else {
            let mut rr = r;
            let mut ans = 0;
            while rr != t {
                rr += 1;
                ans += 1;
                rr %= n;
                if rr == l {
                    break;
                }
            }
            if rr == t {
                val += ans;
                r = t;
                continue;
            }
            ans = 0;
            while r != t {
                r -= 1;
                r += n;
                r %= n;
                ans += 1;
            }
            val += ans;
        }
    }
    println!("{}", val);
}
