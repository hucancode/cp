use std::collections::VecDeque;
impl Solution {
    pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
        let m = nums.len();
        let mut idx: Vec<i32> = nums.into_iter()
            .enumerate()
            .filter_map(|(i, x)| if x == 1 {Some(i as i32)} else {None})
            .collect();
        let n = idx.len();
        //println!("{idx:?}");
        let mut ret = 0;
        if goal == 0 {
            for i in 0..=n {
                let a = if i == 0 {0} else {idx[i-1]+1};
                let b = if i == n {m as i32} else {idx[i]};
                let len = b-a;
                //println!("len {len}");
                ret += (1+len)*len/2;
            }
        } else {
            for i in 0..n as i32 {
                let j = i + goal - 1;
                let i = i as usize;
                let j = j as usize;
                if j >= n {
                    break;
                }
                let left = if i == 0 {idx[i]+1} else {idx[i] - idx[i-1]};
                let right = if j == n-1 {m as i32 - idx[j]} else {idx[j+1] - idx[j]};
                let score = left*right;
                //println!("take {}~{}, gain {left}*{right}={score}", idx[i], idx[j]);
                ret += score;
            }
        }
        return ret as i32;
    }
}
