impl Solution {
    pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = arr.len();
        let mut prefix = vec![0;n+1];
        for i in 1..=n {
            prefix[i] = prefix[i-1] ^ arr[i-1];
        }
        queries.into_iter()
            .map(|lr| (lr[0] as usize, lr[1] as usize))
            .map(|(l,r)| prefix[l] ^ prefix[r+1])
            .collect()
    }
}
