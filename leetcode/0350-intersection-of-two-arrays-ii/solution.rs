impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut v1 = vec![0;1001];
        let mut v2 = vec![0;1001];
        for x in nums1 {
            v1[x as usize] += 1;
        }
        for x in nums2 {
            v2[x as usize] += 1;
        }
        v1.into_iter()
            .zip(v2.into_iter())
            .map(|(a,b)| std::cmp::min(a,b))
            .enumerate()
            .filter_map(|(i,x)| if x > 0 { Some(vec![i as i32; x as usize]) } else { None })
            .flatten()
            .collect()
    }
}
