use std::collections::VecDeque;
use std::iter::FromIterator;
impl Solution {
    pub fn spiral_order(mut matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut matrix = 
            VecDeque::from_iter(
                matrix
                .into_iter()
                .map(|v| VecDeque::from_iter(v.into_iter())));
        let mut ret = Vec::new();
        while !matrix.is_empty() {
            if let Some(q) = matrix.pop_front() {
                ret.extend(q.into_iter());
            }
            for arr in matrix.iter_mut() {
                if let Some(x) = arr.pop_back() {
                    ret.push(x);
                }
            }
            if let Some(q) = matrix.pop_back() {
                ret.extend(q.into_iter().rev());
            }
            for arr in matrix.iter_mut().rev() {
                if let Some(x) = arr.pop_front() {
                    ret.push(x);
                } 
            }
        }
        ret
    }
}