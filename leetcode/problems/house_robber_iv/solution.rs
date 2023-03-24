use std::cmp::max;
impl Solution {
    fn check(nums: &Vec<i32>, cap: i32, requirement: i32) -> bool {
        let n = nums.len();
        let mut f1 = if nums[0] <= cap {1} else {0};
        let mut f2 = if nums[1] <= cap {1} else {0};
        f2 = max(f1, f2);
        for i in 2..n {
            let f = max(f2, if nums[i] <= cap {f1+1} else {f2});
            f1 = f2;
            f2 = f;
            if f2 >= requirement {
                break;
            }
        }
        return f2 >= requirement;
    }
    pub fn min_capability(nums: Vec<i32>, k: i32) -> i32 {
        if(nums.len() < 2) {
            return nums[0];
        }
        let mut l = 1;
        let mut r = *nums.iter().max().unwrap_or(&0);
        while l < r {
            let m = l+ (r-l)/2;
            let good = Self::check(&nums, m, k);
            if good {
                r = m;
            } else {
                l = m+1;
            }
        }
        return l;
    }
}