impl Solution {
    pub fn longest_ideal_string(s: String, k: i32) -> i32 {
        use std::cmp::{min, max};
        let n = s.len();
        let mut f = vec![0; 26];
        for c in s.chars().map(|c| (c as u8 - 'a' as u8) as i32) {
            let mut g = f.clone();
            let l = max(0, c-k);
            let r = min(25, c+k);
            for x in l..=r {
                f[c as usize] = max(f[c as usize], g[x as usize]+1);
            }
            //println!("c = {c}, f = {f:?}");
        }
        //println!("{f:?}");
        f.into_iter().max().unwrap()
    }
}
