use std::cmp::min;
impl Solution {
    pub fn min_cost(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut count = vec![vec![0;n];n+1];
        for i in 1..=n {
            for j in 0..n {
                count[i][j] = count[i-1][j];
            }
            count[i][nums[i-1] as usize] += 1;
        }
        let mut f = vec![k+n as i32;n+1];
        f[0] = 0;
        for i in 1..=n {
            let mut cost: i32 = count[i].iter()
                .filter(|&&c| c > 1)
                .sum();
            let mut count_i = count[i].clone();
            //println!("checking range 0~{i}");
            for j in 0..i {
                f[i] = min(f[i], f[j]+cost+k);
                //println!("split 0~{j} and {j}~{i}, cost = {}+{cost}+{k} = {}", f[j], f[j]+cost+k);
                let x = nums[j] as usize;
                count_i[x] -= 1;
                if count_i[x] > 1 {
                    cost -= 1;
                } else if count_i[x] == 1 {
                    cost -= 2;
                }
                //println!("throw away {x}, trimmed length is now = {cost}");
            }
        }
        return f[n];
    }
}