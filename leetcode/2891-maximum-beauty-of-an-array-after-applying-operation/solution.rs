impl Solution {
    pub fn maximum_beauty(mut nums: Vec<i32>, k: i32) -> i32 {
        use std::cmp::Ordering;
        use std::cmp::max;
        nums.sort();
        (0..nums.len())
            .map(|i| {
                let target = nums[i] + k*2;
                let j = nums.binary_search_by(|x| match x.cmp(&target) {
                    Ordering::Equal => Ordering::Less,
                    order => order,
                }).unwrap_err();
                max(j-i, 1)
            })
            .max()
            .unwrap_or(1) as i32
    }
}
