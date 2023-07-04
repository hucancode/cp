use std::cmp::min;
impl Solution {
    pub fn minimize_concatenated_length(words: Vec<String>) -> i32 {
        const ALPHABET_SIZE: usize = 26;
        let mut f = vec![vec![i32::MAX;ALPHABET_SIZE];ALPHABET_SIZE];
        let words: Vec<(usize,usize,i32)> = words.iter()
            .map(|s| s.as_bytes())
            .map(|s| (s[0], s[s.len()-1], s.len()))
            .map(|(a,b,n)| (a as usize - 'a' as usize, b as usize - 'a' as usize, n as i32))
            .collect();
        let mut ret = 0; 
        let (a,b,n) = words[0];
        f[a][b] = n;
        for (a,b,n) in words.into_iter().skip(1) {
            let prev = f;
            f = vec![vec![i32::MAX;ALPHABET_SIZE];ALPHABET_SIZE];
            for i in 0..ALPHABET_SIZE {
                for j in 0..ALPHABET_SIZE {
                    if prev[i][j] == i32::MAX {
                        continue;
                    }
                    let next = prev[i][j] + n;
                    if j == a {
                        f[i][b] = min(f[i][b], next - 1);
                    } else {
                        f[i][b] = min(f[i][b], next);
                    }
                    if i == b {
                        f[a][j] = min(f[a][j], next - 1);
                    } else {
                        f[a][j] = min(f[a][j], next);
                    }
                }
            }
            //println!("f : {f:?}");
        }
        let mut ret = i32::MAX;
        for i in 0..ALPHABET_SIZE {
            for j in 0..ALPHABET_SIZE {
                ret = min(ret, f[i][j]);
            }
        }
        ret
    }
}