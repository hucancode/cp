impl Solution {
    pub fn buddy_strings(s: String, goal: String) -> bool {
        if goal.len() != s.len() || s.len() < 2 {
            return false;
        }
        let s = s.as_bytes();
        let goal = goal.as_bytes();
        let mut a = '_' as u8;
        let mut b = '_' as u8;
        let mut diff = 0;
        let n = s.len();
        for i in 0..n {
            if s[i] == goal[i] {
                continue;
            }
            if diff == 0 {
                a = s[i];
                b = goal[i];
                diff += 1;
                continue;
            }
            if diff == 1 {
                if b != s[i] || a != goal[i] {
                    return false;
                }
                diff += 1;
                continue;
            }
            return false;
        }
        if diff == 2 {
            return true;
        }
        if diff == 1 {
            return false;
        }
        let mut a = vec![0;26];
        for c in s {
            let key = c - 'a' as u8;
            a[key as usize] += 1;
        }
        a.iter().any(|&x| x > 1)
    }
}