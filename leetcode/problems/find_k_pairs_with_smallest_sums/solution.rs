use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::cmp::Reverse;
impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let n = nums1.len();
        let m = nums2.len();
        let mut q = BinaryHeap::new();
        let mut vis = HashSet::new();
        let i = 0;
        let j = 0;
        let x = Reverse(nums1[i]+nums2[j]);
        q.push((x,i,j));
        while let Some((Reverse(x), i, j)) = q.pop() {
            if vis.contains(&(i,j)) {
                continue;
            }
            vis.insert((i,j));
            if vis.len() >= k as usize {
                break;
            }
            if i < n-1 {
                let x = Reverse(nums1[i+1]+nums2[j]);
                let next = (x, i+1, j);
                q.push(next)
            }
            if j < m-1 {
                let x = Reverse(nums1[i]+nums2[j+1]);
                let next = (x, i, j+1);
                q.push(next)
            }
        }
        vis.into_iter()
            .map(|(i,j)| vec![nums1[i], nums2[j]])
            .collect()
    }
}