impl Solution {
    pub fn is_reachable_at_time(sx: i32, sy: i32, fx: i32, fy: i32, t: i32) -> bool {
      let cost = std::cmp::max((sx - fx).abs(),(sy-fy).abs());
      if cost == 0 && t != 0 {
        return t > 1;
      }
      return cost <= t;
    }
}