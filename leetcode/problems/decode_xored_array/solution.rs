impl Solution {
    pub fn decode(mut encoded: Vec<i32>, first: i32) -> Vec<i32> {
        let mut decoded: Vec<i32> = encoded.iter()
            .scan(first, |acc, &x| {
                *acc ^= x;
                Some(*acc)
            })
            .collect();
        decoded.insert(0, first);
        decoded
    }
}