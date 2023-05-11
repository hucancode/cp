use std::collections::HashSet;
impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        let mut ret = HashSet::new();
        ret.insert(vec![]);
        for x in nums {
            let mut next = ret.clone()
                .into_iter()
                .collect::<Vec<Vec<i32>>>();
            for arr in next.iter_mut() {
                arr.push(x);
            }
            for arr in next.into_iter() {
                ret.insert(arr);
            }
        }
        ret.into_iter()
            .collect::<Vec<Vec<i32>>>()
    }
}