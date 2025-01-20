impl Solution {
    pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
        let n = mat.len();
        let m = mat[0].len();
        let mut rows = vec![0;n*m+1];
        let mut cols = vec![0;n*m+1];
        for i in 0..n {
            for j in 0..m {
                let k = mat[i][j] as usize;
                rows[k] = i;
                cols[k] = j;
            }
        }
        // row 1 row 0
        let mut painted_row = vec![0;n];
        let mut painted_col = vec![0;m];
        for (idx,k) in arr.into_iter().enumerate() {
            let i = rows[k as usize];
            let j = cols[k as usize];
            painted_row[i] += 1;
            painted_col[j] += 1;
            if painted_row[i] == m || painted_col[j] == n {
                return idx as i32;
            }
        }
        return -1;
    }
}
