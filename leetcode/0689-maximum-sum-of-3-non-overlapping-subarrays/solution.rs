impl Solution {
    pub fn max_sum_of_three_subarrays(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::cmp::max;
        let n = nums.len();
        let k = k as usize;
        let mut prefix = vec![0;n+1];
        for i in 1..=n {
            prefix[i] = prefix[i-1] + nums[i-1];
        }
        let mut f1 = vec![0;n+1];
        let mut f2 = vec![0;n+1];
        let mut f3 = vec![0;n+1];
        for i in k..=n {
            f1[i] = max(f1[i-1], prefix[i] - prefix[i-k]);
        }
        for i in k*2..=n {
            f2[i] = max(f2[i-1], prefix[i] - prefix[i-k] + f1[i-k]);
        }
        for i in k*3..=n {
            f3[i] = max(f3[i-1], prefix[i] - prefix[i-k] + f2[i-k]);
        }
        //println!("{f1:?}, {f2:?}, {f3:?}");
        let mut i3 = n;
        while i3 > 1 && f3[i3] == f3[i3-1] {
            i3 -= 1;
        }
        let mut i2 = i3-k;
        while i2 > 1 && f2[i2] <= f2[i2-1] {
            i2 -= 1;
        }
        let mut i1 = i2-k;
        while i1 > 1 && f1[i1] == f1[i1-1] {
            i1 -= 1;
        }
        i1 -= k;
        i2 -= k;
        i3 -= k;
        return vec![i1 as i32, i2 as i32, i3 as i32];
    }
}
