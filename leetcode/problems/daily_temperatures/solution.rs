impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut n = temperatures.len();
        let mut st = Vec::new();
        let mut ret = vec![0;n];
        for (i, x) in temperatures.iter().enumerate() {
            while let Some(&(j, y)) = st.last() {
                if x > y {
                    st.pop();
                    ret[j] = (i-j) as i32;
                } else {
                    break;
                }
            }
            st.push((i, x));
        }
        return ret;
    }
}