use std::cmp::max;
impl Solution {
    pub fn longest_equal_subarray(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let n = nums.len();
        if n < 2 {
            return n as i32;
        }
        let mut occ = vec![Vec::new();n+1];
        let mut ret = 1;
        for i in 0..n {
            let mut l = 0;
            let mut r = i;
            let x = nums[i] as usize;
            occ[x].push(i);
            while l < r {
                let m = (l+r)/2;
                let j = match(occ[x].binary_search(&m)) {
                    Ok(j) => j,
                    Err(j) => j,
                };
                let count = occ[x].len() - j;
                let op = (i - m + 1) - count;
                if(op <= k) {
                    r = m;
                    ret = max(ret, count);
                } else {
                    l = m+1;
                }
            }
        }
        return ret as i32;
    }
}