use std::iter;
impl Solution {
    pub fn maximum_odd_binary_number(s: String) -> String {
        let n = s.chars().filter(|&c| c=='1').count();
        let m = s.len();
        iter::repeat('1').take(n-1)
            .chain(iter::repeat('0').take(m-n))
            .chain(iter::once('1'))
            .collect()
    }
}