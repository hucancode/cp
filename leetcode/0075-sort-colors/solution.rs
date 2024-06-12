impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut colors = vec![0;3];
        for &x in nums.iter() {
            colors[x as usize] += 1;
        }
        let n = nums.len();
        let mut c = 0;
        let mut i = 0;
        while i < n {
            while colors[c] == 0 {
                c += 1;
            }
            nums[i] = c as i32;
            colors[c] -= 1;
            i += 1;
        } 
    }
}
