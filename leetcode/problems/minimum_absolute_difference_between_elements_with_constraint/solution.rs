use std::collections::BinaryHeap;
use std::cmp::min;
impl Solution {
    pub fn min_absolute_difference(nums: Vec<i32>, x: i32) -> i32 {
        let mut pool = Vec::new();
        let mut ret = i32::MAX;
        let n = nums.len();
        let mut x = x as usize;
        //println!("________________");
        for i in x..n {
            let target = nums[i-x];
            if let Err(j) = pool.binary_search(&target) {
                pool.insert(j, target);
            }
            let target = nums[i];
            //println!("find {target} in {pool:?}");
            match(pool.binary_search(&target)) {
                Ok(j) => return 0,
                Err(j) => {
                    if j < pool.len() && j >= 0 {
                        ret = min(ret, (pool[j] - target).abs());
                    }
                    if j < pool.len()-1 {
                        ret = min(ret, (pool[j+1] - target).abs());
                    }
                    if j > 0 {
                        ret = min(ret, (pool[j-1] - target).abs());
                    }
                },
            };
        }
        return ret;
    }
}