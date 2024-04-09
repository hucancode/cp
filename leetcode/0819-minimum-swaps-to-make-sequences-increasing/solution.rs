impl Solution {
    pub fn min_swap(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        use std::cmp::min;
        let INF = 1000_000_000;
        let mut keep_cost = 0; // cost to keep this element
        let mut swap_cost = 1; // cost to swap this element
        for ((a1, b1, a2, b2)) in nums1.windows(2)
            .zip(nums2.windows(2))
            .map(|(w1, w2)| (w1[0], w1[1], w2[0], w2[1])) {
            let mut next_keep_cost = INF;
            let mut next_swap_cost = INF;
            // keep -> keep, or swap -> swap
            if b1 > a1 && b2 > a2 {
                next_keep_cost = min(next_keep_cost, keep_cost);
                next_swap_cost = min(next_swap_cost, swap_cost + 1);
            }
            // swap -> keep or keep -> swap
            if b1 > a2 && b2 > a1 {
                next_keep_cost = min(next_keep_cost, swap_cost);
                next_swap_cost = min(next_swap_cost, keep_cost + 1);
            }
            keep_cost = next_keep_cost;
            swap_cost = next_swap_cost;
        }
        min(keep_cost, swap_cost)
    }
}
