use std::cmp::Ordering;
const INF: i32 = 1000_000_000;
impl Solution {
    fn merge_sort(nums: &mut Vec<i32>, diff: i32) -> i64 {
        let n = nums.len();
        if n <= 1 {
            return 0;
        }
        let mut ret = 0;
        let mut left = nums.clone();
        let mut right = left.split_off(n/2);
        ret += Self::merge_sort(&mut left, diff);
        ret += Self::merge_sort(&mut right, diff);
        let n = left.len();
        let m = right.len();
        for x in left.iter() {
            let target = x - diff;
            let i = right.binary_search_by(|&y| 
                if y >= target {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            ).unwrap_err();
            ret += (m-i) as i64;
        }
        nums.clear();
        let mut i = 0;
        let mut j = 0;
        while i < n || j < m {
            let li = left.get(i).unwrap_or(&INF);
            let rj = right.get(j).unwrap_or(&INF);
            if li < rj {
                nums.push(*li);
                i += 1;
            } else {
                nums.push(*rj);
                j += 1;
            }
        }
        return ret;
    }
    pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, diff: i32) -> i64 {
        let n = nums1.len();
        let mut delta: Vec<i32> = (0..n)
            .map(|i| nums1[i] - nums2[i])
            .collect();
        //println!("delta = {nums:?}");
        // now the problem becomes:
        // find all pairs (i,j), where i<j, delta[i] - delta[j] <= diff
        Self::merge_sort(&mut delta, diff)
    }
}