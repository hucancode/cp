impl Solution {
    pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
        let mut nums: Vec<i32> = grid.into_iter().flatten().collect();
        nums.sort();
        if !nums.iter().all(|k| (k - nums[0])%x == 0) {
            return -1;
        }
        let n = nums.len();
        let mut prefix = vec![0;n+1];
        for i in 1..=n {
            prefix[i] = prefix[i-1]+nums[i-1];
        }
        let mut ret = i32::MAX;
        for i in 1..=n {
            let pivot = nums[i-1];
            let cost_l = pivot*i as i32 - prefix[i];
            let cost_r = prefix[n] - prefix[i] - pivot * (n-i) as i32;
            ret = ret.min((cost_l + cost_r)/x);
        }
        return ret;
    }
}
