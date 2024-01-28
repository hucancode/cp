use std::collections::VecDeque;
use std::cmp::max;
use std::cmp::min;
impl Solution {
    pub fn falling_squares(positions: Vec<Vec<i32>>) -> Vec<i32> {
        let n = positions.len();
        let mut elevation = vec![0;n];
        let mut ret = vec![0;n];
        for i in 0..n {
            let mut e = (0..i).filter(|&j| {
                    let l1 = positions[i][0];
                    let l2 = positions[j][0];
                    let r1 = l1+positions[i][1];
                    let r2 = l2+positions[j][1];
                    max(l1,l2) < min(r1, r2)
                })
                .map(|j| elevation[j])
                .max()
                .unwrap_or(0);
            elevation[i] = e + positions[i][1];
            if i > 0 {
                ret[i] = max(ret[i-1], elevation[i]);
            } else {
                ret[i] = elevation[i];
            }
        }
        return ret;
    }
}