impl Solution {
    pub fn unique_letter_string(s: String) -> i32 {
        let n = s.len();
        let s = s.chars().map(|c| (c as u8 - 'A' as u8) as usize);
        let mut f = vec![0;n+1];
        let mut last_seen = vec![0;26];
        let mut last_last_seen = vec![0;26];
        for (i,c) in s.enumerate() {
            //println!("{last_seen:?}");
            let gain_unique = i + 1 - last_seen[c];
            let loss_unique = last_seen[c] - last_last_seen[c];
            f[i+1] = f[i] + gain_unique - loss_unique;
            last_last_seen[c] = last_seen[c];
            last_seen[c] = i+1;
        }
        //println!("f: {f:?}");
        f.into_iter().sum::<usize>() as i32
    }
}
