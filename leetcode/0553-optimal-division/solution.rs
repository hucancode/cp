impl Solution {
    pub fn optimal_division(nums: Vec<i32>) -> String {
        let nums: Vec<String> = nums.iter().map(i32::to_string).collect();
        if nums.len() < 3 {
            return nums.join("/");
        }
        let mut ret = String::new();
        ret += &nums[0];
        ret += "/(";
        ret += &nums.into_iter()
            .skip(1)
            .collect::<Vec<String>>()
            .join("/");
        ret += ")";
        ret
    }
}
