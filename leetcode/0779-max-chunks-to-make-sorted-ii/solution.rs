impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        use std::cmp::{min, max};
        let n = arr.len();
        let mut min_suffix = vec![i32::MAX; n+1];
        let mut max_prefix = vec![i32::MIN; n];
        max_prefix[0] = arr[0];
        for i in 1..n {
            max_prefix[i] = max(max_prefix[i-1], arr[i]);
        }
        for i in (0..n).rev() {
            min_suffix[i] = min(min_suffix[i+1], arr[i]);
        }
        let mut ret = 0;
        for i in 0..n {
            if max_prefix[i] <= min_suffix[i+1] {
                ret += 1;
            }
        }
        ret
    }
}
