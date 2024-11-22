impl Solution {
    pub fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
        use std::cmp::Ordering;
        use std::cmp::min;
        let n = arr.len();
        let mut l = 1;
        let mut r = n-1;
        while l < n && arr[l-1] <= arr[l] {
            l+=1;
        }
        while r >= l && arr[r] >= arr[r-1] {
            r-=1;
        }
        //println!("at most we can keep [0,{l}), [{r},{n})");
        let mut ret = min(r, n-l);
        for i in 0..l {
            let x = arr[i];
            let j = arr[r..n]
                .binary_search_by(|y| match y.cmp(&x) {
                    Ordering::Equal => Ordering::Greater,
                    order => order,
                })
                .unwrap_err() + r;
            let removed = j-i-1;
            //println!("keep [0,{i}], [{j},{n}) dispose {removed}");
            ret = min(ret, removed);
        }
        ret as i32
    }
}
