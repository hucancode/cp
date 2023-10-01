impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        let data = s.as_bytes()
            .chunks(k as usize)
            .enumerate()
            .flat_map(|(i, arr)| {
                if i%2 == 0 { 
                    arr.iter().copied().rev().collect()
                } else {
                    arr.to_owned()
                }
            })
            .collect();
        String::from_utf8(data).unwrap()
    }
}