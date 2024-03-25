use std::collections::HashMap;
impl Solution {
    pub fn distance(nums: Vec<i32>) -> Vec<i64> {
        let n = nums.len();
        let mut groups: HashMap<i32, Vec<i64>> = HashMap::new();
        let mut ret = vec![0;n];
        for (i,x) in nums.into_iter().enumerate() {
            groups.entry(x).or_default().push(i as i64);
        }
        for g in groups.into_values() {
            //println!("processing {g:?}");
            let n = g.len();
            let mut sum_left = 0;
            let mut sum_right = g.iter().sum::<i64>();
            for (i, x) in g.into_iter().enumerate() {
                let up = (i as i64) * x;
                let down = ((n-i) as i64) * x;
                let score = sum_right - sum_left + up - down;
                //println!("at {x}, score = {sum_right} - {sum_left} + {up} - {down}");
                ret[x as usize] = score;
                sum_left += x as i64;
                sum_right -= x as i64;
            }
        }
        ret
    }
}
