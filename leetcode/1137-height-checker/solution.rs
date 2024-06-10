impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut h2 = heights.clone();
        h2.sort();
        h2.iter()
            .zip(heights.iter())
            .filter(|(x,y)| x != y)
            .count() as i32
    }
}
