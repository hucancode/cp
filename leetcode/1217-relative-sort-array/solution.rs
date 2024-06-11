impl Solution {
    pub fn relative_sort_array(mut arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut f = vec![usize::MAX; 1001];
        for (i, &x) in arr2.iter().enumerate() {
            f[x as usize] = i;
        }
        arr1.sort_by(|&a,&b| match f[a as usize].cmp(&f[b as usize]) {
            std::cmp::Ordering::Equal => a.cmp(&b),
            order => order,
        });
        arr1
    }
}
