impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let s: Vec<usize> = s.chars()
            .map(|c| c as usize - 'a' as usize)
            .collect();
        let n = s.len();
        let mut f = vec![0;n];
        let mut g = vec![0;n];
        for i in 1..n {
            let c = s[i-1];
            f[i] = f[i-1] | (1<<c);
        }
        for i in (0..n-1).rev() {
            let c = s[i+1];
            g[i] = g[i+1] | (1<<c);
        }
        let mut score_mask = vec![0i32;26];
        for i in 1..n-1 {
            let c = s[i];
            let mask = f[i] & g[i];
            score_mask[c] = score_mask[c] | mask;
        }
        score_mask
            .iter()
            .fold(0, |acc, x| acc + x.count_ones()) as i32
    }
}