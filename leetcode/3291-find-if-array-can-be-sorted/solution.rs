impl Solution {
    pub fn can_sort_array(nums: Vec<i32>) -> bool {
        use std::cmp::{min, max};
        let mut max_so_far = i32::MIN;
        let mut local_max = i32::MIN;
        let mut local_min = i32::MAX;
        let mut bit_count = nums[0].count_ones();
        for x in nums {
            if x.count_ones() != bit_count {
                max_so_far = max(max_so_far, local_max);
                local_min = x;
                local_max = x;
                bit_count = x.count_ones();
            }
            local_max = max(local_max, x);
            local_min = min(local_min, x);
            if local_min < max_so_far {
                return false;
            }
        }
        return true;
    }
}
