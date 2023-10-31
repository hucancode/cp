impl Solution {
    pub fn find_array(mut pref: Vec<i32>) -> Vec<i32> {
      for i in (1..pref.len()).rev() {
        pref[i] ^= pref[i-1];
      }
      return pref;
    }
}