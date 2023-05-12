use std::cmp::max;
impl Solution {
    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        let n = questions.len();
        let mut f = vec![0i64;n]; // best score if we take question i
        let mut g = vec![0i64;n]; // best score if we don't take question i
        let mut ret = 0;
        for (i, q) in questions.iter().enumerate() {
            if i > 0 {
                g[i] = max(g[i], g[i-1]);
            }
            f[i] = max(f[i], g[i] + q[0] as i64);
            let j = i + 1 + q[1] as usize;
            if j < n {
                g[j] = max(g[j], f[i]);
            }
            ret = max(ret, f[i]);
        }
        //println!("{f:?}");
        ret
    }
}