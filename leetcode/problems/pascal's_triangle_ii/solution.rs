impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let row_index = row_index as usize;
        let mut f = vec![0; 1+row_index];
        let mut n = 0;
        while n <= row_index {
            let mut k = 1;
            for i in 0..=n {
                if(i == 0 || i == n) {
                    f[i] = 1;
                    continue;
                }
                let nextk = f[i];
                f[i] += k;
                k = nextk;
            }
            n+=1;
        }
        return f;
    }
}