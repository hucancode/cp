use std::cmp::max;
use std::collections::VecDeque;

impl Solution {
    pub fn max_compatibility_sum(students: Vec<Vec<i32>>, mentors: Vec<Vec<i32>>) -> i32 {
        let n = students.len();
        let m = students[0].len();
        let mut score = vec![vec![0;n];n];
        for i in 0..n {
            for j in 0..n {
                for q in 0..m {
                    if students[i][q] == mentors[j][q] {
                        score[i][j] += 1;
                    }
                }
            }
        }
        let mut q = VecDeque::new();
        q.push_back((-1, 0, 0));
        let mut ret = 0;
        while let Some((i, mask, x)) = q.pop_front() {
            ret = max(ret, x);
            let i = i+1;
            for j in 0..n {
                if (mask & 1<<j) != 0 {
                    continue;
                }
                let mask = mask | 1<<j;
                let mut x = x + score[i as usize][j];
                q.push_back((i, mask, x));
            }
        }
        return ret;
    }
}