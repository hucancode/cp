impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let n = s.len();
        let mut count = vec![0;26];
        let mut prefix = Vec::new();
        for c in s.chars() {
            let c = (c as u8 - 'a' as u8) as usize;
            count[c] += 1;
            prefix.push(count.clone());
        }
        let mut ret = Vec::new();
        let mut prev = -1;
        for i in 0..n {
            let mut can_cut = (0..26).all(|j| prefix[i][j] == prefix[n-1][j] || prefix[i][j] == 0);
            if can_cut {
                ret.push(i as i32 - prev);
                prev = i as i32;
            }
        }
        return ret;
    }
}
