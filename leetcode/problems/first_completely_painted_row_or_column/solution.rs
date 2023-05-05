impl Solution {
    pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
        let n = mat.len();
        let m = mat[0].len();
        let mut rows = vec![0;n];
        let mut cols = vec![0;m];
        let mut coords = vec![(0,0);m*n+1];
        for (x, row) in mat.iter().enumerate() {
            for (y, &v) in row.iter().enumerate() {
                coords[v as usize] = (x,y);
            }
        }
        for (i, &v) in arr.iter().enumerate() {
            let (x, y) = coords[v as usize];
            rows[x] += 1;
            cols[y] += 1;
            if rows[x] == m || cols[y] == n {
                return i as i32;
            }
        }
        return 0;
    }
}