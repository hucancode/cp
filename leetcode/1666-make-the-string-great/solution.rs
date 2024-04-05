impl Solution {
    pub fn make_good(s: String) -> String {
        let mut st: Vec<char> = Vec::new();
        for c in s.chars() {
            if let Some(&x) = st.last() {
                if x != c && x.eq_ignore_ascii_case(&c) {
                    st.pop();
                    continue;
                }
            }
            st.push(c);
        }
        st.into_iter().collect()
    }
}
