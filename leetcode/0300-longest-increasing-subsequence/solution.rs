impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        use std::cmp::max;
        let mut ret = 0;
        let mut lis = Vec::new();
        for x in nums {
            let i = lis.partition_point(|&y| y < x);
            if i >= lis.len() {
                lis.push(x);
            } else {
                lis[i] = x;
            }
            ret = max(ret, i+1);
        }
        ret as i32
    }
}
