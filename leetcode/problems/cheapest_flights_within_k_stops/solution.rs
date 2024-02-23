use std::collections::BinaryHeap;
use std::cmp::Reverse;
impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let n = n as usize;
        let src = src as usize;
        let dst = dst as usize;
        let k = k as usize;
        let mut adj = vec![Vec::new(); n];
        for flight in flights {
            let u = flight[0] as usize;
            let v = flight[1] as usize;
            let p = flight[2];
            adj[u].push((v,p));
        }
        let mut vis = vec![vec![false;k+2];n];
        let mut q = BinaryHeap::new();
        q.push((Reverse(0), 0, src));
        while let Some((Reverse(price), stop, u)) = q.pop() {
            if vis[u][stop] {
                continue;
            }
            vis[u][stop] = true;
            if u == dst {
                return price;
            }
            if stop > k {
                continue;
            }
            for &(v,puv) in adj[u].iter() {
                q.push((Reverse(price + puv), stop + 1, v));
            }
        }
        return -1;
    }
}