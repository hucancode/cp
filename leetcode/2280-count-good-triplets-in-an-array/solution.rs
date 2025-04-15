impl Solution {
    pub fn good_triplets(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let mut ret = 0;
        let n = nums1.len();
        let mut indices = vec![0;n];
        for (i, x) in nums1.into_iter().enumerate() {
            indices[x as usize] = i;
        }
        let mut vis = Vec::with_capacity(n);
        for (i, y) in nums2.into_iter().enumerate() {
            let j = indices[y as usize];
            let k = vis.binary_search(&j).unwrap_err();
            vis.insert(k, j);
            let x_count = k;
            let checked = i;
            let to_check = n-1-j;
            let z_count = to_check-(checked-x_count);
            ret += x_count as i64 * z_count as i64;
            //println!("take {y} as y, there are {x_count} element can be used as x, {z_count} element can be used as z. vis {vis:?}");
        }
        return ret;
    }
}
