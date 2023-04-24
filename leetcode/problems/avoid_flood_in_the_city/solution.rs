use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BinaryHeap;

impl Solution {
    pub fn avoid_flood(rains: Vec<i32>) -> Vec<i32> {
        let n = rains.len();
        let mut lakes = HashMap::new();
        for (i, &rain) in rains.iter().rev().enumerate() {
            if rain == 0 {
                continue;
            }
            lakes.entry(rain)
                .or_insert(Vec::new())
                .push(i);
        }
        let mut water = HashSet::new();
        let mut heap = BinaryHeap::new();
        let mut invalid = false;
        let mut ans = rains
            .iter()
            .map(|x| {
                if *x == 0 {
                    if let Some((_, x)) = heap.pop() {
                        water.remove(&x);
                        return x;
                    }
                    return 1;
                }
                let arr = lakes
                    .entry(*x)
                    .or_default();
                if let Some(j) = arr.pop() {
                    if water.contains(&x) {
                        invalid = true;
                        return -1;
                    }
                    water.insert(x);
                }
                if let Some(next) = arr.last() {
                    let i = n-next-1;
                    heap.push((*next, rains[i]));
                }
                return -1;
            })
            .collect();
        if invalid {
            Vec::new()
        } else { 
            ans
        }
    }
}