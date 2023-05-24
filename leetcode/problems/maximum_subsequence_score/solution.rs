use std::cmp::max;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
impl Solution {
    pub fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let n = nums1.len();
        let mut arr: Vec<(i64, i64)> = (0..n)
            .map(|i| (nums2[i] as i64, nums1[i] as i64))
            .collect();
        arr.sort_by(|a,b|b.cmp(&a));
        //println!("{arr:?}");
        let mut ret = 0;
        let mut sum = 0;
        let mut heap = BinaryHeap::new();
        for i in 0..n {
            let (x, y) = arr[i];
            sum += y;
            heap.push(Reverse(y));
            if(heap.len() > k) {
                if let Some(Reverse(y)) = heap.pop() {
                    sum -= y;
                }
            }
            if heap.len() < k {
                continue;
            }
            let score = x * sum;
            ret = max(ret, score);
            //println!("check {x}, {y}, sum = {sum}, score = {score}, ret = {ret}");
        }
        ret
    }
}