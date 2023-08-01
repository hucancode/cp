impl Solution {
    pub fn average_value(nums: Vec<i32>) -> i32 {
        let it = nums.iter().filter(|&&x| x%2 == 0 && x%3 == 0);
        let n = it.clone().count() as i32;
        if n == 0 {
            return 0;
        }
        let x = it.fold(0, |acc, x| acc + x);
        return x/n;
    }
}