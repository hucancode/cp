use std::cmp::Reverse;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
impl Solution {
    pub fn total_cost(costs: Vec<i32>, mut k: i32, candidates: i32) -> i64 {
        const INF: i32 = 1000_000_000;
        let candidates = candidates as usize;
        let mut ret: i64 = 0;
        let mut q: VecDeque<i32> = costs.into_iter().collect();
        let mut left = BinaryHeap::new();
        let mut right = BinaryHeap::new();
        for i in 0..candidates {
            if let Some(x) = q.pop_front() {
                left.push(Reverse(x));
            }
            if let Some(x) = q.pop_back() {
                right.push(Reverse(x));
            }
        }
        while k > 0 {
            let x = left.peek().map_or(INF, |&x| x.0);
            let y = right.peek().map_or(INF, |&x| x.0);
            if x <= y {
                ret += x as i64;
                left.pop();
                if let Some(x) = q.pop_front() {
                    left.push(Reverse(x));
                }
            } else {
                ret += y as i64;
                right.pop();
                if let Some(x) = q.pop_back() {
                    right.push(Reverse(x));
                }
            }
            k -= 1;
        }
        ret
    }
}