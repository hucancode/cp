use std::cmp::min;
use std::cmp::Reverse;
use std::collections::HashSet;
use std::collections::BinaryHeap;
impl Solution {
    pub fn min_days(n: i32) -> i32 {
        let mut vis = HashSet::new();
        let mut q = BinaryHeap::new();
        q.push((Reverse(0), Reverse(n)));
        while let Some((Reverse(cost), Reverse(n))) = q.pop() {
            if(vis.contains(&n)) {
                continue;
            }
            vis.insert(n);
            if n == 0 {
                return cost;
            }
            q.push((Reverse(cost+1), Reverse(n-1)));
            if n%2 == 0 {
                q.push((Reverse(cost+1), Reverse(n/2)));
            }
            if n%3 == 0 {
                q.push((Reverse(cost+1), Reverse(n/3)));
            }
        }
        return n;
    }
}