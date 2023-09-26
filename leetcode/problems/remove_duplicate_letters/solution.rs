impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let mut count = vec![0;26];
        let mut vis = vec![false;26];
        for c in s.chars() {
            let c = c as usize - 'a' as usize;
            count[c] += 1;
        }
        let mut ret = Vec::new();
        for c in s.chars() {
            let c = c as usize - 'a' as usize;
            count[c] -= 1;
            if vis[c] {
                continue;
            }
            while let Some(&x) = ret.last() {
                if x < c || count[x] <= 0 {
                    break;
                }
                vis[x] = false;
                ret.pop();
            }
            ret.push(c);
            vis[c] = true;
        }
        let ret = ret.into_iter()
            .map(|c| (c as u8 +'a' as u8) as char)
            .collect::<String>();
        return ret;
    }
}