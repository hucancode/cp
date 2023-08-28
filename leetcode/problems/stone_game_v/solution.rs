use std::cmp::max;
impl Solution {
    pub fn stone_game_v(a: Vec<i32>) -> i32 {
        let n = a.len();
        let mut prefix = vec![0;n+1];
        for i in 1..=n {
            prefix[i] = prefix[i-1]+a[i-1];
        }
        let mut f = vec![vec![0;n];n];
        for len in 2..=n {
            for j in 1..n {
                let i = j+1-len;
                for mid in i..j {
                    //println!("calculate f[{i}][{j}], try to split [{i}~{mid}], ({mid}~{j}]");
                    let left_sum = prefix[mid+1] - prefix[i];
                    let right_sum = prefix[j+1] - prefix[mid+1];
                    //println!("left sum = {left_sum}, right sum = {right_sum}");
                    if left_sum <= right_sum {
                        f[i][j] = max(f[i][j], left_sum + f[i][mid]);
                    }
                    if right_sum <= left_sum {
                        f[i][j] = max(f[i][j], right_sum + f[mid+1][j]);
                    }
                }
            }
        }
        return f[0][n-1];
    }
}