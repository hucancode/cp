impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut vis = vec![false;101];
        let mut i = nums.len();
        for &x in nums.iter().rev() {
            if vis[x as usize] {
                break;
            }
            vis[x as usize] = true;
            i -= 1;
        }
        return (i as i32 + 2)/3;
    }
}
