impl Solution {
    pub fn shopping_offers(price: Vec<i32>, special: Vec<Vec<i32>>, needs: Vec<i32>) -> i32 {
        use std::collections::{BinaryHeap, HashSet};
        use std::cmp::Reverse;
        let n = price.len();
        let mut q = BinaryHeap::new();
        q.push((Reverse(0), vec![0;n]));
        let cap = price.iter().zip(needs.iter()).map(|(a,b)|a*b).sum::<i32>();
        let mut vis = HashSet::new();
        while let Some((Reverse(cost), items)) = q.pop() {
            if vis.contains(&items) {
                continue;
            }
            let success = items.iter().zip(needs.iter()).all(|(a,b)| a == b);
            if success {
                return cost;
            }
            vis.insert(items.clone());
            for i in 0..n {
                if items[i] >= needs[i] {
                    continue;
                }
                let mut next = items.clone();
                next[i] += 1;
                q.push((Reverse(cost + price[i]), next));
            }
            for offer in special.iter() {
                if cost + offer[n] > cap {
                    continue;
                }
                let mut next = vec![0;n];
                let mut overbought = false;
                for i in 0..n {
                    next[i] = items[i] + offer[i];
                    if next[i] > needs[i] {
                        overbought = true;
                        break;
                    }
                }
                if overbought {
                    continue;
                }
                q.push((Reverse(cost + offer[n]), next));
            }
        }
        return cap;
    }
}
