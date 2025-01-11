impl Solution {
    pub fn can_construct(s: String, k: i32) -> bool {
        let mut count = vec![0;26];
        let k = k as usize;
        let n = s.len();
        if n < k {
            return false;
        }
        for c in s.chars() {
            let i = c as usize - 'a' as usize;
            count[i] += 1;
        }
        let odd = count.into_iter().filter(|c| c%2 != 0).count();
        if odd > k {
            return false;
        }
        return true;
    }
}
