impl Solution {
    pub fn can_see_persons_count(heights: Vec<i32>) -> Vec<i32> {
        let n = heights.len();
        let mut f: Vec<usize> = Vec::new();
        let mut ans = vec![0; n];

        for i in (0..n).rev() {
            let mut count = 0;
            while let Some(&j) = f.last() {
                if heights[i] >= heights[j] {
                    f.pop();
                    count += 1;
                } else {
                    count += 1;
                    break;
                }
            }
            f.push(i);
            ans[i] = count;
        }
        return ans;
    }
}