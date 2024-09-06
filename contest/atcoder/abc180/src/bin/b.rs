use proconio::input;

fn main() {
    input! {
        x: [f64],
    }
    let mut manhattan: f64 = 0.0;
    let mut euclid: f64 = 0.0;
    let mut chebyshev: f64 = 0.0;
    for &i in &x {
        manhattan += i.abs();
        euclid += i*i;
        chebyshev = chebyshev.max(i.abs());
    }
    euclid = euclid.sqrt();
    println!("{}", manhattan);
    println!("{}", euclid);
    println!("{}", chebyshev);
}
