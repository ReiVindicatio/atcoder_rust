use proconio::{input, marker::Usize1};
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
    }
    let mut g = vec![vec![]; n];
    for (u, v) in uv {
        g[u].push(v);
    }
    let mut dist = vec![None; n];
    dist[0] = Some(0);
    let mut q = VecDeque::new();
    q.push_back(0);
    while let Some(v) = q.pop_front() {
        for &e in g[v].iter() {
            if e == 0 && v != 0 {
                println!("{}", dist[v].unwrap() + 1);
                return;
            } else if dist[e].is_none() {
                q.push_back(e);
                dist[e] = Some(dist[v].unwrap() + 1);
            }
        }
    }
    println!("-1");
}
