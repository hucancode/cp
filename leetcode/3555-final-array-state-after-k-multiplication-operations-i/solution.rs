impl Solution {
    pub fn get_final_state(mut nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
        use std::collections::BinaryHeap;
        use std::cmp::Reverse;
        let mut q: BinaryHeap<(Reverse<i32>, Reverse<usize>)> = nums.iter()
            .enumerate()
            .map(|(i, &x)| (Reverse(x), Reverse(i)))
            .collect();
        for _ in 0..k {
            let (_, Reverse(i)) = q.pop().unwrap();
            nums[i] *= multiplier;
            q.push((Reverse(nums[i]), Reverse(i)));
        }
        nums
    }
}
