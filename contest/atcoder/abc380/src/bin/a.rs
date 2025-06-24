use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut n: Chars,
    }
    n.sort();
    if n[0] == '1' && n[1] == '2' && n[2] == '2' && n[3] == '3' && n[4] == '3' && n[5] == '3' {
        println!("Yes");
    }  else {
        println!("No");
    }
}
