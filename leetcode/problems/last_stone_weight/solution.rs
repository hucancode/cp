use std::collections::BinaryHeap;
impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::from(stones);
        loop {
            match (heap.pop(), heap.pop()) {
                (Some(a), Some(b)) => heap.push(a - b),
                (Some(a), None) => return a,
                _ => break
            }
        }
        return 0;
    }
}