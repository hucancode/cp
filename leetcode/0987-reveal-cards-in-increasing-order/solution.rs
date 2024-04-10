impl Solution {
    pub fn deck_revealed_increasing(mut deck: Vec<i32>) -> Vec<i32> {
        use std::collections::VecDeque;
        deck.sort_by(|a,b|b.cmp(&a));
        let n = deck.len();
        let mut ret = vec![0;n];
        let mut q: VecDeque<usize> = (0..n).collect();
        let mut take = true;
        while let Some(i) = q.pop_front() {
            if take {
                if let Some(x) = deck.pop() {
                    ret[i] = x;
                }
            } else {
                q.push_back(i);
            }
            take = !take;
        }
        ret
    }
}
