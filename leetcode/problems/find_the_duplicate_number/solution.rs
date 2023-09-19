impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut vis = std::collections::HashSet::new();
        for x in nums {
            if !vis.insert(x) {
                return x;
            }
        }
        return 0;
    }
}