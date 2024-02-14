impl Solution {
    pub fn count_matching_subarrays(nums: Vec<i32>, pattern: Vec<i32>) -> i32 {
        let n = nums.len()-1;
        let m = pattern.len();
        let mut diagram = vec![0;n];
        for i in 0..n {
            diagram[i] = (nums[i+1] - nums[i]).signum();
        }
        let mut lps = vec![0;m];
        // lps[i] = longest string that is both prefix and suffix of pattern[0~i]
        // pattern = abc123abc -> lps[8] = 3 
        let mut len = 0;
        let mut i = 1;
        while i < m {
            if (pattern[i] == pattern[len]) {
                len += 1;
                lps[i] = len;
                i += 1;
            } else {
                if (len != 0) {
                    len = lps[len - 1];
                } else {
                    lps[i] = 0;
                    i += 1;
                }
            }
        }
        // kmp search using lps computed above
        let mut ret = 0;
        let mut i = 0;
        let mut j = 0;
        while j < n {
            let matched = pattern[i] == diagram[j];
            if (matched) {
                i += 1;
                j += 1;
            }
            if (i == m) {
                ret += 1;
                i = lps[i - 1];
            } else if j < n && pattern[i] != diagram[j] {
                if (i != 0) {
                    i = lps[i - 1];
                } else {
                    j += 1;
                }
            }
        }
        return ret;
    }
}