impl Solution {
    pub fn is_good(nums: Vec<i32>) -> bool {
        let n = nums.len() - 1;
        let mut occ = vec![0;n+1];
        for x in nums {
            let x = x as usize;
            if x > n {
                return false;
            }
            occ[x] += 1;
            if (x != n && occ[x] > 1) || (x == n && occ[x] > 2) {
                return false;
            }
        }
        return true;
    }
}