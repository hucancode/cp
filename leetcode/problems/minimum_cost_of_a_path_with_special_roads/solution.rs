use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BinaryHeap;
use std::cmp::Reverse;
use std::cmp::min;
impl Solution {
    pub fn minimum_cost(start: Vec<i32>, target: Vec<i32>, special_roads: Vec<Vec<i32>>) -> i32 {
        let mut adj = HashMap::new();
        let mut nodes = HashSet::new();
        let x0 = start[0];
        let y0 = start[1];
        let xn = target[0];
        let yn = target[1];
        nodes.insert((x0,y0));
        nodes.insert((xn,yn));
        for edge in special_roads {
            let xa = edge[0];
            let ya = edge[1];
            let xb = edge[2];
            let yb = edge[3];
            let d = edge[4];
            nodes.insert((xa,ya));
            nodes.insert((xb,yb));
            adj.entry((xa, ya, xb, yb))
                .and_modify(|x| *x = min(*x, d))
                .or_insert(d);
        }
        let mut q = BinaryHeap::new();
        q.push((Reverse(0), x0, y0));
        while let Some((Reverse(du), xu, yu)) = q.pop() {
            let u = (xu, yu);
            if u == (xn, yn) {
                return du;
            }
            if !nodes.contains(&u) {
                continue;
            }
            nodes.remove(&u);
            for &(xv, yv) in nodes.iter() {
                let mut duv = (xu - xv).abs() + (yu - yv).abs();
                if let Some(&d) = adj.get(&(xu, yu, xv, yv)) {
                    duv = min(duv, d);
                }
                q.push((Reverse(du + duv), xv, yv));
            }
        }
        return 0;
    }
}