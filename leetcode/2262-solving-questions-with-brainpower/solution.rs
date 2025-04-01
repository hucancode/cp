impl Solution {
    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        let n = questions.len();
        let mut f = vec![0;n+1];
        for i in (0..n).rev() {
            let p = questions[i][0] as i64;
            let j = questions[i][1] as usize;
            if i + j >= n {
                f[i] = f[i+1].max(p);
            } else {
                f[i] = f[i+1].max(f[i+j+1] + p);
            }
        }
        f[0]
    }
}
