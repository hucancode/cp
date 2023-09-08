impl Solution {
    pub fn max_sum_min_product(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut prefix = vec![0;n+1];
        for i in 1..=n {
            prefix[i] = prefix[i-1] + nums[i-1] as i64;
        }
        let mut sum = vec![0;n];
        let mut st = Vec::new();
        for i in 0..n {
            while let Some(&j) = st.last() {
                if nums[i] <= nums[j] {
                    st.pop();
                } else {
                    break;
                }
            }
            let j = if st.is_empty() {0} else {*st.last().unwrap_or(&0) + 1};
            sum[i] += prefix[i+1] - prefix[j];
            st.push(i);
        }
        st.clear();
        for i in (0..n).rev() {
            while let Some(&j) = st.last() {
                if nums[i] <= nums[j] {
                    st.pop();
                } else {
                    break;
                }
            }
            let j = if st.is_empty() {n} else {*st.last().unwrap_or(&n)};
            sum[i] += prefix[j] - prefix[i+1];
            st.push(i);
        }
        let ret = (0..n).map(|i| sum[i]*nums[i] as i64).max().unwrap_or(0);
        return (ret % 1000_000_007) as i32;
    }
}