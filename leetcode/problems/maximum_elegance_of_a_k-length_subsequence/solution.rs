use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BinaryHeap;
use std::cmp::max;
use std::cmp::min;
use std::cmp::Reverse;
impl Solution {
    pub fn find_maximum_elegance(mut items: Vec<Vec<i32>>, mut k: i32) -> i64 {
        let mut k = k as usize;
        let mut total = 0;
        let mut groups = HashMap::new();
        let mut used = HashSet::new();
        let mut candidates = BinaryHeap::new();
        items.sort_by(|a,b|b[0].cmp(&a[0]));
        for i in 0..k {
            let p = items[i][0];
            let cat = items[i][1];
            groups.entry(cat)
                .and_modify(|top_profit| {
                    candidates.push(Reverse(min(p, *top_profit)));
                    *top_profit = max(*top_profit, p);
                })
                .or_insert(p);
            total += p as i64;
            used.insert(cat);
        }
        //println!("items = {items:?}, total = {total}, used = {used:?}, candidates = {candidates:?}");
        let mut ret = total + used.len()  as i64 * used.len() as i64;
        for i in k..items.len() {
            let p = items[i][0];
            let cat = items[i][1];
            if used.contains(&cat) {
                continue;
            }
            if let Some(Reverse(x)) = candidates.pop() {
                used.insert(cat);
                total -= x as i64;
                total += p as i64;
            }
            let score = total + used.len() as i64 * used.len() as i64;
            ret = max(ret, score);
        }
        return ret;
    }
}