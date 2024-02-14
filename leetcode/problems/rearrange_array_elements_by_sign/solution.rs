impl Solution {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let a = nums.iter().filter(|&&x| x > 0);
        let b = nums.iter().filter(|&&x| x < 0);
        a.zip(b)
            .flat_map(|(&x,&y)| vec![x,y].into_iter())
            .collect()
    }
}