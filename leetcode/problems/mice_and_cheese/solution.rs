use std::collections::BinaryHeap;
use std::cmp::Reverse;
impl Solution {
    pub fn mice_and_cheese(reward1: Vec<i32>, reward2: Vec<i32>, k: i32) -> i32 {
        let mut q = BinaryHeap::new();
        let mut ret = 0;
        let n = reward1.len();
        for i in 0..n {
            let d = reward1[i] - reward2[i];
            ret += reward1[i];
            q.push(Reverse(d));
            if q.len() > k as usize {
                if let Some(Reverse(d)) = q.pop() {
                    ret -= d;
                }
            }
        }
        return ret;
    }
}