use std::collections::HashSet;
impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let k = nums[0].len();
        let n = 1<<k;
        let nums: HashSet<usize> = nums.iter()
            .map(|s| usize::from_str_radix(s, 2).unwrap())
            .collect();
        for i in 0..n {
            if nums.contains(&i) {
                continue;
            }
            return (0..k)
                .map(|b| if (i>>b)&1 == 1 {'1'} else {'0'})
                .rev()
                .collect::<String>();
        }
        String::from("")
    }
}