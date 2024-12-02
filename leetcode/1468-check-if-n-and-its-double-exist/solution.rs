impl Solution {
    pub fn check_if_exist(mut arr: Vec<i32>) -> bool {
        arr.sort();
        (0..arr.len()).any(|i| 
            arr.binary_search(&(2*arr[i]))
                .is_ok_and(|j| j!=i)
        )
    }
}
