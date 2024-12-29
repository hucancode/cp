impl Solution {
    pub fn num_ways(words: Vec<String>, target: String) -> i32 {
        let INF = 1000_000_007;
        let target: Vec<u8> = target.chars()
            .map(|c| c as u8 - 'a' as u8)
            .collect();
        let words: Vec<Vec<u8>> = words.into_iter()
            .map(|w| w.chars()
                .map(|c| c as u8 - 'a' as u8)
                .collect())
            .collect();
        let m = words.iter().map(|w| w.len()).max().unwrap();
        let n = target.len();
        let mut count = vec![vec![0;26];m];
        for i in 0..m {
            for w in words.iter() {
                if i >= w.len() {
                    continue;
                }
                let c = w[i] as usize;
                count[i][c] += 1;
            }
        }
        let mut f = vec![vec![0;m+1];n+1];
        // f[i][j] = how many ways to build target [0,i) using words with index [0,j)
        for j in 0..=m {
            f[0][j] = 1;
        }
        for i in 1..=n {
            for j in 1..=m {
                let c = target[i-1] as usize;
                let use_c = (count[j-1][c] as u64)*f[i-1][j-1];
                let ignore_c = f[i][j-1];
                f[i][j] = use_c + ignore_c;
                f[i][j] %= INF;
            }
        }
        //println!("{f:?}");
        return f[n][m] as i32;
    }
}
