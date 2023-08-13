use std::cmp::max;
impl Solution {
    pub fn max_sum(nums: Vec<i32>) -> i32 {
        let nums: Vec<(i32,i32)> = nums.into_iter()
            .map(|x| {
                let mut y = x;
                let mut ret = 0;
                while y != 0 {
                    ret = max(ret, y%10);
                    y /= 10;
                }
                return (x,ret);
            })
            .collect();
        println!("{nums:?}");
        let mut ret = -1;
        let n = nums.len();
        for i in 0..n {
            for j in i+1..n {
                let (a,b) = nums[i];
                let (c,d) = nums[j];
                if b == d {
                    ret = max(ret, a+c);
                }
            }
        }
        return ret;
    }
}