impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let n = heights.len();
        let mut left = vec![0;n];
        let mut right = vec![0;n];
        let mut st = Vec::new();
        for (i, &h) in heights.iter().enumerate() {
            while let Some(&j) = st.last() {
                if heights[j] >= h {
                    st.pop();
                } else {
                    break;
                }
            }
            if let Some(&j) = st.last() {
                left[i] = i - j;
            } else {
                left[i] = i+1;
            }
            st.push(i);
        }
        //println!("{left:?}");
        st.clear();
        for (i, &h) in heights.iter().enumerate().rev() {
            while let Some(&j) = st.last() {
                if heights[j] >= h {
                    st.pop();
                } else {
                    break;
                }
            }
            if let Some(&j) = st.last() {
                right[i] = j - i;
            } else {
                right[i] = n-i;
            }
            st.push(i);
        }
        //println!("{right:?}");
        left.into_iter()
            .zip(right.into_iter())
            .zip(heights.into_iter())
            .map(|((l, r), h)| h*(l+r-1) as i32)
            .max()
            .unwrap_or(0)
    }
}
