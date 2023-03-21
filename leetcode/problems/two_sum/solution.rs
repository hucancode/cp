impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let n = nums.len();
        for i in 0..n {
            let x = nums[i];
            let (_, suffix) = nums.split_at(i+1);
            match suffix
                .iter()
                .position(|y| x+y == target) {
                    Some(j) => return vec![i as i32, (i+j+1) as i32],
                    None => continue,
                };
        }
        return vec![0,0];
    }
}