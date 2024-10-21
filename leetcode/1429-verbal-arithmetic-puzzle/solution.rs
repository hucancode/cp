impl Solution {
    pub fn is_solvable(words: Vec<String>, result: String) -> bool {
        use std::collections::{HashMap, HashSet};
        let mut weight = HashMap::new();
        let mut leads = HashSet::new();
        let mut update_weight = |w: String, mut k: i32| {
            for c in w.chars().rev() {
                weight.entry(c)
                    .and_modify(|w| *w += k)
                    .or_insert(k);
                k *= 10;
            }
            if w.len() > 1 {
                for c in w.chars().take(1) {
                    leads.insert(c);
                }
            }
        };
        for w in words {
            update_weight(w, 1);
        }
        update_weight(result, -1);
        let mut weight: Vec<(i32, bool)> = weight.into_iter()
            .map(|(k,v)| (v, leads.contains(&k)))
            .collect();
        weight.sort();
        println!("weight = {weight:?}");
        let n = weight.len();
        let m = n/2;
        let mut sum: HashMap<i32, HashSet<i32>> = HashMap::new();
        let mut used_digit = vec![false;10];
        let mut slot = 0;
        let mut q: Vec<(i32, bool)> = Vec::new();
        for i in 0..=9 {
            q.push((i, false));
        }
        let mut mask = 0;
        let mut value = 0;
        while let Some((d, revert)) = q.pop() {
            if revert {
                used_digit[d as usize] = false;
                slot -= 1;
                //println!("revert {d} at slot {slot}, value before revert = {value}");
                let (w, _) = weight[slot];
                mask ^= 1<<d;
                value -= w*d;
                continue;
            }
            //println!("use {d} at slot {slot}, value before use = {value}");
            let (w, is_lead) = weight[slot];
            used_digit[d as usize] = true;
            mask |= 1<<d;
            value += w*d;
            slot += 1;
            q.push((d, true));
            if is_lead && d == 0 {
                continue;
            }
            if value > 0 {
                continue;
            }
            if slot >= m {
                sum.entry(value)
                    .and_modify(|set| { set.insert(mask); })
                    .or_insert(HashSet::from([mask]));
                continue;
            }
            for i in (0..=9).filter(|&i| !used_digit[i as usize]) {
                q.push((i, false));
            }
        }
        if n == 1 {
            return sum.contains_key(&0);
        }
        for i in (0..=9).rev() {
            q.push((i, false));
        }
        used_digit = vec![false;10];
        slot = m;
        mask = 0;
        value = 0;
        while let Some((d, revert)) = q.pop() {
            if revert {
                used_digit[d as usize] = false;
                slot -= 1;
                let (w, _) = weight[slot];
                mask ^= 1<<d;
                value -= w*d;
                continue;
            }
            //println!("use {d} at slot {slot}, value before use = {value}");
            let (w, is_lead) = weight[slot];
            used_digit[d as usize] = true;
            mask |= 1<<d;
            value += w*d;
            slot += 1;
            q.push((d, true));
            if is_lead && d == 0 {
                continue;
            }
            if slot >= n {
                if value >= 0 && sum.get(&-value)
                    .is_some_and(|set| set.iter().any(|m| m & mask == 0)) {
                    return true;
                }
                continue;
            }
            for i in (0..=9).filter(|&i| !used_digit[i as usize]) {
                q.push((i, false));
            }
        }
        return false;
    }
}
