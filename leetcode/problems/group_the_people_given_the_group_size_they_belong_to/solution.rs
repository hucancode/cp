use std::collections::HashMap;
impl Solution {
    pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        let mut map: HashMap<usize, Vec<i32>> = HashMap::new();
        let mut ret = vec![];
        for (i, n) in group_sizes.iter().enumerate() {
            map.entry(*n as usize)
                .or_default()
                .push(i as i32);
        }
        for (n, arr) in map.iter() {
            let mut i = 0;
            while i < arr.len() {
                let group = arr
                    .clone()
                    .into_iter()
                    .skip(i)
                    .take(*n)
                    .collect();
                ret.push(group);
                i += n;
            }
        }
        return ret;
    }
}