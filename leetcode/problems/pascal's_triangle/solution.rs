impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut ret: Vec<Vec<i32>> = Vec::new();
        for n in 1..=num_rows {
            let n = n as usize;
            let mut row = vec![1;n];
            for i in 1..n-1 {
                row[i] = ret[n-2][i] + ret[n-2][i-1];
            }
            ret.push(row);
        }
        return ret;
    }
}