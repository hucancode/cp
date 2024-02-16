use std::collections::{HashSet, VecDeque};
impl Solution {
    pub fn split_array_same_average(mut nums: Vec<i32>) -> bool {
        let n = nums.len();
        let total: i32 = nums.iter().sum();
        let mut f = vec![vec![HashSet::<i32>::new();n+1];n];
        // f[i][j] = use exact `i` nums in the range [0,j)
        // which are the possible sum could we have
        f[0][0].insert(0);
        for i in 1..n {
            for j in i..=n {
                f[i][j] = f[i][j-1].clone();
                for x in f[i-1][j-1].clone() {
                    f[i][j].insert(x+nums[j-1]);
                }
            }
        }
        for m in 1..n {
            // if we pick exact `m` items to make A, what is the sum of A that we need to satisfy?
            if total*m as i32 % n as i32 != 0 {
                continue;
            }
            let target = total*m as i32 / n as i32;
            // we need to pick exact `m` item, to sum up to exact `target` 
            for j in m..=n {
                if f[m][j].contains(&target) {
                    return true;
                }
            }
        }
        return false;
    }
}