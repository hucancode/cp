impl Solution {
    pub fn min_operations(mut nums: Vec<i32>, target: i32) -> i32 {
        let mut count = vec![0;33];
        for x in nums {
            let p = (x as f64).log2() as usize;
            count[p] += 1;
        }
        let mut ret = 0;
        let n = 32;
        for i in 0..n {
            if target & (1<<i) != 0 {
                let mut j = i;
                while j < n && count[j] == 0 {
                    j += 1;
                }
                if j == n {
                    return -1;
                }
                while j > i {
                    count[j] -= 1;
                    count[j-1] += 2;
                    ret += 1;
                    j -= 1;
                }
                count[i] -= 1;
            }
            count[i+1] += count[i]/2;
        }
        return ret;
    }
}