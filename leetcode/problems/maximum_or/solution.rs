impl Solution {
    pub fn maximum_or(mut nums: Vec<i32>, k: i32) -> i64 {
        let mut left: Vec<i64> = nums.iter()
            .scan(0, |acc, x| {*acc |= x; Some(*acc)})
            .map(|x| x as i64)
            .collect();
        left.insert(0, 0);
        let mut right: Vec<i64> = nums.iter()
            .rev()
            .scan(0, |acc, x| {*acc |= x; Some(*acc)})
            .map(|x| x as i64)
            .collect();
        right.reverse();
        right.push(0);
        let best = nums.iter()
            .map(|x| x.leading_zeros())
            .min()
            .unwrap_or(0);
        nums.iter()
            .enumerate()
            .filter(|(_, x)| x.leading_zeros() == best)
            .map(|(i, &x)| 
                (x as i64)<<k | left[i] | right[i+1]
            )
            .max()
            .unwrap_or(0)
    }
}