impl Solution {
    pub fn h_index(mut citations: Vec<i32>) -> i32 {
        citations.sort_by(|a, b| b.cmp(&a));
        let n = citations.len();
        let mut ret = 0;
        for i in 0..n {
            let k = citations[i];
            if k-1 < i as i32 {
                break;
            }
            ret = i+1;
        }
        ret as i32
    }
}