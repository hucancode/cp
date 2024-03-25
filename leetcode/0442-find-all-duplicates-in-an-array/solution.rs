impl Solution {
    pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
        let mut ret = Vec::new();
        let n = nums.len();
        for i in 0..n {
            let mut x = nums[i] as usize;
            nums[i] = 0;
            while x != 0 && x != nums[x-1] as usize {
                let next = nums[x-1] as usize;
                nums[x-1] = x as i32;
                x = next;
            }
            if x != 0 {
                ret.push(x as i32);
            }
        }
        ret
    }
}
