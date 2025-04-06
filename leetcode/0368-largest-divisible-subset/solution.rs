impl Solution {
    pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort();
        let n = nums.len();
        let mut f = vec![1;n];
        let mut prev = vec![n;n];
        let mut best = 0;
        for i in 0..n {
            for j in 0..i {
                if nums[i] % nums[j] == 0 && f[j] + 1 > f[i] {
                    f[i] = f[j]+1;
                    prev[i] = j;
                }
            }
            if f[i] > f[best] {
                best = i;
            }
        }
        let mut ret = vec![nums[best]];
        while prev[best] != n {
            best = prev[best];
            ret.push(nums[best]);
        }
        return ret;
    }
}
