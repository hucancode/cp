use std::collections::HashMap;
use std::collections::VecDeque;
use std::cmp::{min, max};
impl Solution {
    pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut f: HashMap<i32, VecDeque<usize>> = HashMap::new();
        let mut j = 0;
        let mut ret = 0;
        let k = k as usize;
        for (i, x) in nums.into_iter().enumerate() {
            let arr = f.entry(x).or_default();
            arr.push_back(i);
            if arr.len() > k {
                if let Some(j2) = arr.pop_front() {
                    j = max(j, j2+1);
                }
            }
            //println!("at {i} with num = {x}, indices = {arr:?}");
            ret = max(ret, i-j+1);
        }
        return ret as i32;
    }
}
