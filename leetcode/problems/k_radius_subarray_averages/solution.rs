impl Solution {
    pub fn get_averages(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let n = nums.len();
        let m = k*2+1;
        let mut ret = vec![-1;n];
        if n < m {
            return ret;
        }
        let mut prefix = vec![0;n+1];
        for i in 1..=n {
            prefix[i] = prefix[i-1] + nums[i-1] as i64;
        }
        for i in k..n-k {
            let x = prefix[i+k+1] - prefix[i-k];
            ret[i] = (x/m as i64) as i32;
        }
        ret
    }
}