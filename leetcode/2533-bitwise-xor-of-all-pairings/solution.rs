impl Solution {
    pub fn xor_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n = nums1.len();
        let m = nums2.len();
        let mut ret = 0;
        if m%2 != 0 {
            ret ^= nums1.into_iter().fold(0, |acc, x| acc^x);
        }
        if n%2 != 0 {
            ret ^= nums2.into_iter().fold(0, |acc, x| acc^x);
        }
        ret
    }
}
