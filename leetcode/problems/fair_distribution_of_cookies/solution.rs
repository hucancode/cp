use std::cmp::{min, max};
impl Solution {
    fn solve(cookies: &Vec<i32>, mask: i32, k: usize) -> i32 {
        let n = cookies.len();
        if k == 1 {
            return cookies.iter()
                .enumerate()
                .filter_map(|(i,x)| if 1<<i & mask == 0 {Some(x)} else {None})
                .fold(0, |acc, &x| acc+x);
        }
        let taken = mask.count_ones() as usize;
        if n-taken <= k {
            return cookies.iter()
                .enumerate()
                .filter_map(|(i,x)| if 1<<i & mask == 0 {Some(x)} else {None})
                .fold(0, |acc, &x| max(acc, x));
        }
        let m = 1<<n;
        let mut ret = i32::MAX;
        for next in 1..m {
            if next & mask != 0 {
                continue;
            }
            let taken = taken + next.count_ones() as usize;
            if n-taken < k {
                continue;
            }
            let a = cookies.iter()
                .enumerate()
                .filter_map(|(i,x)| if 1<<i & next != 0 {Some(x)} else {None})
                .fold(0, |acc, x| acc+x);
            if a >= ret {
                continue;
            }
            let b = Self::solve(cookies, mask | next, k-1);
            ret = min(ret, max(a,b));
        }
        return ret;
    }
    pub fn distribute_cookies(cookies: Vec<i32>, k: i32) -> i32 {
        Self::solve(&cookies, 0, k as usize)
    }
}