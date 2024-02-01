impl Solution {
    pub fn divide_array(mut nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        nums.sort();
        let iter = nums.chunks(3);
        if iter.clone()
            .all(|a| a.iter().max().unwrap() - a.iter().min().unwrap() <= k)  {
            iter.map(|a| a.to_vec())
                .collect()
        } else {
            Vec::new()
        }
    }
}