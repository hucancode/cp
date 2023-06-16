impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let n = citations.len();
        let mut l = 0;
        let mut r = n;
        while l < r {
            let m = (l+r)/2;
            let good = citations[m] >= (n - m) as i32;
            if good {
                r = m;
            } else {
                l = m+1;
            }
        }
        (n-r) as i32
    }
}