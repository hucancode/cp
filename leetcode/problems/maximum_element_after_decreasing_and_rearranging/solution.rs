impl Solution {
    pub fn maximum_element_after_decrementing_and_rearranging(mut arr: Vec<i32>) -> i32 {
        arr.sort();
        arr.into_iter().fold(0, |acc,x| std::cmp::min(acc+1, x))
    }
}