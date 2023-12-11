impl Solution {
    pub fn find_special_integer(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        for i in 0..n {
            if arr[i] == arr[i+n/4] {
                return arr[i];
            }
        }
        return -1;
    }
}