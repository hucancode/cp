impl Solution {
    pub fn sort_by_bits(mut arr: Vec<i32>) -> Vec<i32> {
        arr.sort_by(|a,b| {
          let pa = a.count_ones();
          let pb = b.count_ones();
          if(pa == pb) {
            return a.cmp(&b);
          }
          return pa.cmp(&pb);
        });
        return arr;
    }
}