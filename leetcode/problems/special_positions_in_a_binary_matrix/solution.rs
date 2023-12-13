impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let n = mat.len();
        let m = mat[0].len();
        let mut cols = vec![0;n];
        let mut rows = vec![0;m];
        for i in 0..n {
            for j in 0..m {
                cols[i] += mat[i][j];
                rows[j] += mat[i][j];
            }
        }
        let mut ret = 0;
        for i in 0..n {
            for j in 0..m {
                if cols[i] == 1 && 
                    rows[j] == 1 && 
                    mat[i][j] == 1 {
                    ret += 1;
                }
            }
        }
        return ret;
    }
}