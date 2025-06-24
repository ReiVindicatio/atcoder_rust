use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [i32; n],
    }
    let mut st = vec![];
    let mut ans = vec![];
    for h in h.iter().rev() {
        ans.push(st.len());
        while let Some(&x) = st.last() {
            if x > h {
                break;
            }
            st.pop();
        }
        st.push(h);
    }
    for i in ans.iter().rev() {
        print!("{} ", i);
    }
    println!();
}
