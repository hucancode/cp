use std::collections::HashSet;
impl Solution {
    pub fn find_difference(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let nums1: HashSet<i32> = nums1.into_iter().collect();
        let nums2: HashSet<i32> = nums2.into_iter().collect();
        let a: Vec<i32> = nums1.difference(&nums2)
            .cloned()
            .collect();
        let b: Vec<i32> = nums2.difference(&nums1)
            .cloned()
            .collect();
        return vec![a,b];
    }
}