impl Solution {
    pub fn max_score(nums: Vec<i32>) -> i32 {
        let mut negative: Vec<i32> = nums
            .iter()
            .filter(|&&x| x <= 0)
            .cloned()
            .collect();
        negative.sort_by(|a, b| b.cmp(a));
        let positive: Vec<i32> = nums
            .iter()
            .filter(|&&x| x > 0)
            .cloned()
            .collect();
        let mut sum = positive.iter().fold(0, |acc, x| acc + *x as i64);
        let mut ret = positive.len() as i32;
        for &x in negative.iter() {
            sum += x as i64;
            if(sum <= 0) {
                break;
            }
            ret += 1;
        }
        return ret;
    }
}