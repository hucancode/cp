impl Solution {
    pub fn find_diagonal_order(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut n = nums.iter()
            .enumerate()
            .map(|(i,row)| row.len() + i)
            .max()
            .unwrap_or(0);
        let mut groups = vec![Vec::new();n];
        for (i, row) in nums.into_iter().enumerate().rev() {
            for (j, x) in row.into_iter().enumerate() {
                groups[i+j].push(x);
            }
        }
        groups.into_iter()
            .flatten()
            .collect()
    }
}