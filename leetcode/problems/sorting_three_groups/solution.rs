use std::cmp::min;
impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut f = vec![0;3];
        for x in nums {
            let g = f.clone();
            for j in 1..=3 {
                let cost = if x==j as i32 {0} else {1};
                let mut prev = *g.iter().take(j).min().unwrap_or(&0);
                f[j-1] = prev + cost;
            }
            //println!("{f:?}");
        }
        return f.into_iter().min().unwrap_or(0);
    }
}