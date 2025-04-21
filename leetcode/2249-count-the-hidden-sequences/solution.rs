impl Solution {
    pub fn number_of_arrays(differences: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let n = differences.len();
        let mut arr0 = vec![0;n+1];
        let mut x = 0;
        let mut y = 0;
        for i in 0..n {
            arr0[i+1] = arr0[i] + differences[i] as i64;
            x = x.min(arr0[i+1]);
            y = y.max(arr0[i+1]);
        }
        (upper as i64 - lower as i64 + x - y + 1).max(0) as i32
    }
}
