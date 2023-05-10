use std::collections::VecDeque;
impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut ret = vec![vec![0;n];n];
        let mut matrix: VecDeque<VecDeque<(usize, usize)>> = (0..n)
            .map(|r| {
                (0..n)
                    .map(|c| (r, c))
                    .collect()
            })
            .collect();
        let mut k = 1;
        while !matrix.is_empty() {
            if let Some(q) = matrix.pop_front() {
                for (x,y) in q.into_iter() {
                    ret[x][y] = k;
                    k += 1;
                }
            }
            for arr in matrix.iter_mut() {
                if let Some((x, y)) = arr.pop_back() {
                    ret[x][y] = k;
                    k += 1;
                }
            }
            if let Some(q) = matrix.pop_back() {
                for (x,y) in q.into_iter().rev() {
                    ret[x][y] = k;
                    k += 1;
                }
            }
            for arr in matrix.iter_mut().rev() {
                if let Some((x, y)) = arr.pop_front() {
                    ret[x][y] = k;
                    k += 1;
                } 
            }
        }
        ret
    }
}