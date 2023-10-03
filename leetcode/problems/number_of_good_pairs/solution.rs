impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut group = vec![0;101];
        for x in nums {
            group[x as usize] += 1;
        }
        group.iter().fold(0, |acc,&x| acc+(1..x).sum::<i32>())
    }
}