impl Solution {
    pub fn search_distinct(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let mut r = n-1;
        let mut l = 0;
        while l < r {
            let m = (l+r)/2;
            let good = nums[m] < nums[r];
            if good {
                r = m;
            } else {
                l = m+1;
            }
        }
        if target > nums[n-1] {
            r = if l == 0 {n-1} else {l-1};
            l = 0;
        } else {
            r = n-1;
        }
        while l < r {
            let m = (l+r)/2;
            let good = nums[m] >= target;
            if good {
                r = m;
            } else {
                l = m+1;
            }
        }
        if nums[l] != target {
            return -1;
        }
        return l as i32;
    }
    pub fn search(mut nums: Vec<i32>, target: i32) -> bool {
        nums.dedup();
        Self::search_distinct(nums, target) != -1
    }
}