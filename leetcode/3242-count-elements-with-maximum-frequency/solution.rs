impl Solution {
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        let mut f = vec![0;101];
        for x in nums {
            f[x as usize] += 1;
        }
        let mut k = 0;
        let mut count = 0;
        for x in f {
            if x > k {
                k = x;
                count = 1;
            } else if x == k {
                count += 1;
            }
        }
        return k * count;
    }
}
