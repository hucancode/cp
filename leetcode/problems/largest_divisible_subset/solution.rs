use std::cmp::max;
impl Solution {
    pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort();
        let n = nums.len();
        let mut best_i = 0;
        let mut f = vec![0; n];
        let mut prev = vec![-1;n];
        for i in 0..n {
            let mut k = 1;
            f[i] += 1;
            //println!("f[{i}] = {}", f[i]);
            if f[i] > f[best_i] {
                best_i = i;
            }
            loop {
                //println!("search {}*{k}={}", nums[i], nums[i]*k);
                let j1 = max(i+1, nums.partition_point(|&x| x < nums[i]*k));
                let j2 = max(j1, nums.partition_point(|&x| x <= nums[i]*k));
                if j1 >= n {
                    break;
                }
                //println!("update from {j1}~{j2} with {f:?}");
                for j in j1..j2 {
                    if nums[j] != nums[i]*k {
                        break;
                    }
                    if f[j] < f[i] {
                        f[j] = f[i];
                        //println!("f[{j}] = {}", f[i]);
                        prev[j] = i as i32;
                    }
                }
                if j2 >= n {
                    break;
                }
                k = max(k+1, nums[j2]/nums[i]);
            }
        }
        let mut ret = vec![nums[best_i]];
        while prev[best_i] != -1 {
            best_i = prev[best_i] as usize;
            ret.push(nums[best_i]);
        }
        ret
    }
}