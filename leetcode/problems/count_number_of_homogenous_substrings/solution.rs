impl Solution {
    pub fn count_homogenous(s: String) -> i32 {
        let INF: i64 = 1000_000_007;
        let s = s.as_bytes();
        let count = |n| n*(n+1)/2;
        let mut ret = 0;
        let mut streak = 1;
        let n = s.len();
        for i in 1..=n {
            if i == n || s[i] != s[i-1] { 
                ret += count(streak as i64);
                ret %= INF;
                streak = 1;
            } else {
                streak += 1;
            }
        }
        ret as i32
    }
}