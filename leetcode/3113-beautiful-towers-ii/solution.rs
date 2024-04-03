impl Solution {
    pub fn maximum_sum_of_heights(max_heights: Vec<i32>) -> i64 {
        let n = max_heights.len();
        let mut f = vec![0;n+1];
        let mut g = vec![0;n+1];
        let mut st = Vec::new();
        st.push((0, 0));
        for i in 0..n {
            let x = max_heights[i] as i64;
            while let Some(&(_, y)) = st.last() {
                if x > y {
                    break;
                }
                st.pop();
            }
            if let Some(&(j, _)) = st.last() {
                f[i+1] = x*(i-j+1) as i64 + f[j];
            }
            st.push((i+1, x));
        }
        st.clear();
        st.push((n, 0));
        for i in (0..n).rev() {
            let x = max_heights[i] as i64;
            while let Some(&(_, y)) = st.last() {
                if x > y {
                    break;
                }
                st.pop();
            }
            if let Some(&(j, _)) = st.last() {
                g[i] = x*(j-i) as i64 + g[j];
            }
            st.push((i, x));
        }
        (0..n).map(|i| g[i] + f[i])
            .max()
            .unwrap_or(0)
    }
}
