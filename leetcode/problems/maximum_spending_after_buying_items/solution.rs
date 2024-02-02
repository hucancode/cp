use std::collections::BinaryHeap;
use std::cmp::Reverse;
impl Solution {
    pub fn max_spending(mut values: Vec<Vec<i32>>) -> i64 {
        let mut candidates = BinaryHeap::new();
        let mut ret = 0;
        for i in 0..values.len() {
            if let Some(x) = values[i].pop() {
                candidates.push((Reverse(x), i));
            }
        }
        let mut day = 1;
        while let Some((Reverse(x), i)) = candidates.pop() {
            //println!("on day {day}, buy from shop {i} price {x}");
            ret += day*x as i64;
            day += 1;
            if let Some(x) = values[i].pop() {
                candidates.push((Reverse(x), i));
            }
        }
        return ret;
    }
}