use std::cmp::Ordering;
impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
        let mut potions = potions;
        potions.sort();
        let n = potions.len();
        let mut pairs = vec![0;spells.len()];
        for (i, &spell) in spells.iter().enumerate() {
            let count = n - match potions.binary_search_by(|&p| 
                match (p as i64 * spell as i64).cmp(&success) {
                    Ordering::Equal => Ordering::Greater,
                    order => order,
                }) {
                Ok(x) => x,
                Err(x) => x,
            };
            pairs[i] = count as i32;
        }
        return pairs;
    }
}