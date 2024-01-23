use std::collections::VecDeque;
use std::cmp::max;
impl Solution {
    pub fn max_length(arr: Vec<String>) -> i32 {
        let arr: Vec<i32> = arr.into_iter()
            .filter_map(|s| {
                let mut ret = 0;
                for c in s.chars() {
                    let k = c as u8 - 'a' as u8;
                    let x = 1<<k;
                    if x & ret != 0 {
                        return None;
                    }
                    ret |= x;
                }
                Some(ret)
            })
            .collect();
        let mut ret = 0;
        let mut q: VecDeque<(usize, i32)> = arr.clone()
            .into_iter()
            .enumerate()
            .collect();
        let n = arr.len();
        while let Some((i, mask)) = q.pop_front() {
            ret = max(ret, mask.count_ones() as i32);
            for j in i+1..n {
                if arr[j] & mask != 0 {
                    continue;
                }
                q.push_back((j, mask | arr[j]));
            }
        }
        ret
    }
}