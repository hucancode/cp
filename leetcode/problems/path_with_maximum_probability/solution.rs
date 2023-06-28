use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::cmp::Ordering;

#[derive(PartialEq, PartialOrd)]
struct NonNanF64(f64);
impl Eq for NonNanF64 {}
impl Ord for NonNanF64 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

impl Solution {
    pub fn max_probability(n: i32, edges: Vec<Vec<i32>>, succ_prob: Vec<f64>, start: i32, end: i32) -> f64 {
        let mut adj: HashMap<i32, Vec<(i32, f64)>> = HashMap::new();
        for (i,e) in edges.into_iter().enumerate() {
            let u = (e[0], succ_prob[i]);
            let v = (e[1], succ_prob[i]);
            adj.entry(e[0])
                .and_modify(|arr| arr.push(v))
                .or_insert(vec![v]);
            adj.entry(e[1])
                .and_modify(|arr| arr.push(u))
                .or_insert(vec![u]);
        }
        let mut q = BinaryHeap::new();
        let mut vis = vec![false;n as usize];
        q.push((NonNanF64(1.0), start));
        while let Some((NonNanF64(du), u)) = q.pop() {
            if vis[u as usize] {
                continue;
            }
            vis[u as usize] = true;
            if u == end {
                return du;
            }
            if let Some(arr) = adj.get(&u) {
                for (v, duv) in arr.iter() {
                    q.push((NonNanF64(duv*du), *v));
                }
            }
        }
        return 0.0;
    }
}