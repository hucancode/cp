impl Solution {
    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        ((k-1).count_ones()%2) as i32
    }
}