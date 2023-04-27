use std::collections::HashMap;

impl Solution {
    pub fn distance(nums: Vec<i32>) -> Vec<i64> {
        let n = nums.len();
        let mut map: HashMap<i32, Vec<usize>> = HashMap::new();
        let mut left = vec![0i64;n];
        for (i, x) in nums.iter().enumerate() {
            let arr = map.entry(*x).or_default();
            if let Some(j) = arr.last() {
                let delta = (i - j) as i64;
                left[i] = left[*j] + delta * arr.len() as i64;
            }
            arr.push(i);
        }
        let mut map: HashMap<i32, Vec<usize>> = HashMap::new();
        let mut right = vec![0i64;n];
        for (i, x) in nums.iter().rev().enumerate() {
            let i = n-1-i;
            let arr = map.entry(*x).or_default();
            if let Some(j) = arr.last() {
                let delta = (j - i) as i64;
                right[i] = right[*j] + delta * arr.len() as i64;
            }
            arr.push(i);
        }
        return left.iter().zip(right.iter())
            .map(|(a, b)| a+b)
            .collect();
    }
}