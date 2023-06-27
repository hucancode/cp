use std::cmp::max;
use std::cmp::Ordering::{Greater, Less};
use std::collections::HashMap;
impl Solution {
    pub fn smallest_distance_pair(mut nums: Vec<i32>, mut k: i32) -> i32 {
        let n = nums.len();
        nums.sort();
        let m = (nums[n-1] - nums[0]) as usize;
        let mut delta = vec![0;m + 1];
        let mut count = HashMap::new();
        let mut i = 0;
        while i < n {
            let next = nums.binary_search_by(|&x| 
                if x > nums[i] { Greater } else { Less }
            ).unwrap_err();
            let c = next - i;
            count.entry(nums[i])
                .and_modify(|x| *x += c)
                .or_insert(c);
            i = next;
        }
        let mut it1 = count.iter();
        while let Some((i,ci)) = it1.next() {
            delta[0] += ci*(ci-1)/2;
            let mut it2 = it1.clone();
            while let Some((j, cj)) = it2.next() {
                let x = (j - i).abs();
                delta[x as usize] += cj*ci;
            }
        }
        let mut ret = 0;
        while ret < m {
            k -= delta[ret] as i32;
            if k <= 0 {
                break;
            }
            ret += 1;
        }
        ret as i32
    }
}