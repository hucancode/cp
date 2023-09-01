use std::cmp::max;
impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut f = vec![vec![0;n];n];
        // f(i,j) = if we only play in the range [i,j], what is the maximum score
        for len in 0..n {
            for i in 0..n-len {
                let j = i+len;
                for x in i..=j {
                    // in the range [i,j], if we burst balloon x last, what is the maximum score?
                    let mut score = nums[x];
                    if i > 0 {
                        score *= nums[i-1];
                    }
                    if j < n-1 {
                        score *= nums[j+1];
                    }
                    if x > i {
                        score += f[i][x-1];
                    }
                    if x < j {
                        score += f[x+1][j];
                    }
                    f[i][j] = max(f[i][j], score);
                }
            }
        }
        return f[0][n-1];
    }
}