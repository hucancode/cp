impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        let mut l = 1 as usize;
        let mut r = arr.len() - 2;
        while l < r {
            let m = ((l+r)/2) as usize;
            let a = *arr.get(m-1).unwrap_or(&0);
            let b = *arr.get(m).unwrap_or(&0);
            let c = *arr.get(m+1).unwrap_or(&0);
            if a < b && b < c {
                l = m + 1;
            } else if a > b && b > c {
                r = m;
            } else {
                l = m;
                break;
            }
        }
        return l as i32;
    }
}