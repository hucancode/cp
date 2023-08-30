impl Solution {
    pub fn minimum_replacement(nums: Vec<i32>) -> i64 {
        let mut ret = 0;
        let n = nums.len();
        let mut x = nums[n-1];
        for i in (0..n-1).rev() {
            let y = nums[i];
            let k = y/x + if y%x == 0 {0} else {1};
            ret += (k - 1) as i64;
            x = y/k;
            //println!("k = {k}, x = {x}");
        }
        return ret;
    }
}