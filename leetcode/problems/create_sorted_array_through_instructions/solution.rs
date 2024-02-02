impl Solution {
    pub fn create_sorted_array(instructions: Vec<i32>) -> i32 {
        let mut nums = Vec::with_capacity(instructions.len());
        let mut ret = 0;
        for x in instructions {
            let i = nums.partition_point(|&y| y < x);
            let j = nums.len() - nums.partition_point(|&y| y <= x);
            nums.insert(i, x);
            ret += std::cmp::min(i, j);
            ret %= 1000_000_007;
        }
        return ret as i32;
    }
}