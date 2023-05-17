impl Solution {
    pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let f: Vec<i32> = arr.iter()
            .scan(0, |acc, &x| {
                *acc ^= x;
                Some(*acc)
            })
            .collect();
        queries.iter()
            .map(|arr| (arr[0] as usize, arr[1] as usize))
            .map(|(l, r)| f[r] ^ if l > 0 {f[l-1]} else {0})
            .collect()
    }
}