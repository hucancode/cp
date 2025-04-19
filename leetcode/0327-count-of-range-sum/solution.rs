impl Solution {
    pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let mut prevs = Vec::with_capacity(nums.len()+1);
        prevs.push(0);
        let mut sum = 0i64;
        let mut ret = 0;
        for x in nums {
            sum += x as i64;
            let l = sum - upper as i64;
            let r = sum - lower as i64;
            let i = prevs.partition_point(|&x| x < l);
            let j = prevs.partition_point(|&x| x <= r);
            ret += (j - i) as i32;
            let idx = match prevs.binary_search(&sum) {
                Err(i) => i,
                Ok(i) => i,
            };
            prevs.insert(idx, sum);
        }
        return ret;
    }
}
