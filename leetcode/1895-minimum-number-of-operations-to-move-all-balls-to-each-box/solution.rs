impl Solution {
    pub fn min_operations(boxes: String) -> Vec<i32> {
        let n = boxes.len();
        let boxes = boxes.as_bytes();
        let mut prefix = vec![0;n+1];
        let mut suffix = vec![0;n+1];
        let mut cost_left = vec![0;n+1];
        let mut cost_right = vec![0;n+1];
        for i in 1..=n {
            prefix[i] = prefix[i-1] + if boxes[i-1] == '1' as u8 {1} else {0}
        }
        for i in (0..n).rev() {
            suffix[i] = suffix[i+1]+if boxes[i] == '1' as u8 {1} else {0}
        }
        for i in 1..=n {
            cost_left[i] = cost_left[i-1] + prefix[i-1]
        }
        for i in (0..n).rev() {
            cost_right[i] = cost_right[i+1] + suffix[i+1]
        }
        //println!("{cost_left:?}, {cost_right:?}");
        (0..n).map(|i| cost_left[i+1] + cost_right[i]).collect()
    }
}
