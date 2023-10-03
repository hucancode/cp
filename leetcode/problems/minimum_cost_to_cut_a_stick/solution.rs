impl Solution {
    pub fn min_cost(n: i32, mut cuts: Vec<i32>) -> i32 {
        cuts.insert(0, 0);
        cuts.push(n);
        cuts.sort();
        let m = cuts.len();
        let mut f = vec![vec![100_000_000; m];m];
        for i in 0..m-1 {
            f[i][i] = 0;
            f[i][i+1] = 0;
        }
        for len in 2..m {
            for i in 0..m-len {
                let j = i+len;
                let fixed_cost = cuts[j]-cuts[i];
                for k in 1+i..j {
                    let score = f[i][k]+f[k][j]+fixed_cost;
                    f[i][j] = std::cmp::min(f[i][j], score);
                }
                //println!("to cover {}~{} cost {} ", cuts[i], cuts[j], f[i][j]);
            }
        }
        return f[0][m-1];
    }
}