impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        use std::collections::HashMap;
        let n = nums.len();
        let k = k as usize;
        let mut ret = 0;
        let mut sum = 0;
        let mut occ = HashMap::new();
        let mut dup_count = 0;
        for i in 0..n {
            let x = nums[i];
            //println!("check {x}");
            occ.entry(x)
                .and_modify(|c| *c += 1)
                .or_insert(1);
            if occ.get(&x).is_some_and(|&x| x > 1) {
                dup_count += 1;
                // println!("dup {x}, total dup {dup_count}");
            }
            sum += x as i64;
            if i >= k {
                let y = nums[i-k];
                //println!("uncheck {y}");
                occ.entry(y)
                    .and_modify(|c| *c -= 1);
                if occ.get(&y).is_some_and(|&y| y > 0) {
                    dup_count -= 1;
                    // println!("remove dup {y}, total dup {dup_count}");
                }
                sum -= y as i64;
            }
            if i >= k - 1 && dup_count == 0 {
                ret = ret.max(sum);
            }
        }
        ret
    }
}
