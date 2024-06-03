impl Solution {
    pub fn min_operations(target: Vec<i32>, arr: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        use std::cmp::max;
        let n = target.len();
        // convert all x of arr to the position of x in target
        let mut idx: HashMap<i32, usize> = target.into_iter()
            .enumerate()
            .map(|(i,x)| (x,i))
            .collect();
        let mut len = 0;
        let arr = arr.into_iter()
            .filter_map(|x| idx.get(&x));
        // find arr's LIS, the answer is n - LIS's length
        let mut lis = Vec::new();
        for x in arr {
            let i = lis.partition_point(|&y| y < x);
            if i >= lis.len() {
                lis.push(x);
            } else {
                lis[i] = x;
            }
            len = max(len, i+1);
        }
        return (n - len) as i32;
    }
}
