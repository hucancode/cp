use std::cmp::min;

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut c1 = vec![0;1001];
        let mut c2 = vec![0;1001];
        for x in nums1 {
            c1[x as usize] += 1;
        }
        for x in nums2 {
            c2[x as usize] += 1;
        }
        c1.iter().zip(c2.iter())
            .map(|(a,b)| min(a,b))
            .enumerate()
            .map(|(i, &x)| vec![i as i32;x])
            .flatten()
            .collect()
    }
}