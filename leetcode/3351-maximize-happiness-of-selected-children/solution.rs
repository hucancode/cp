impl Solution {
    pub fn maximum_happiness_sum(mut happiness: Vec<i32>, mut k: i32) -> i64 {
        happiness.sort();
        let n = happiness.len();
        let mut ret = 0i64;
        let mut j = 0;
        while let Some(x) = happiness.pop() {
            if x <= j || k == 0 {
                break;
            }
            ret += x as i64 - j as i64;
            j += 1;
            k -= 1;
        }
        ret
    }
}
