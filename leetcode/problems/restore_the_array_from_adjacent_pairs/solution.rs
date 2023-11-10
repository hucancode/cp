use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn restore_array(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
        let mut adj: HashMap<i32, HashSet<i32>> = HashMap::new();
        for a in adjacent_pairs {
            let u = a[0];
            let v = a[1];
            adj.entry(u)
                .or_default()
                .insert(v);
            adj.entry(v)
                .or_default()
                .insert(u);
        }
        let mut ret = Vec::new();
        if let Some((u, au)) = adj.iter().find(|(u,au)| au.len() == 1) {
            let mut u = *u;
            loop {
                ret.push(u);
                let au = adj.entry(u).or_default();
                if let Some(v) = au.iter().next().cloned() {
                    let av = adj.entry(v).or_default();
                    av.remove(&u);
                    u = v;
                } else {
                    break;
                }
            }
        }
        return ret;
    }
}