impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ret = vec![vec![]];
        for x in nums {
            let mut next = ret.clone();
            for arr in next.iter_mut() {
                arr.push(x);
            }
            ret.extend(next);
        }
        ret
    }
}